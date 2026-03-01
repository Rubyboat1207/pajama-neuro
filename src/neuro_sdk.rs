use futures_util::{SinkExt, StreamExt};
use neuro_sama::game::Api;
use schemars::JsonSchema;
use serde::Deserialize;
use std::{
    sync::{Arc, Mutex, OnceLock},
    time::Duration,
};
use tokio::sync::mpsc::{self, UnboundedSender};

use crate::{
    game::{engine::ScummEngine, events::init_hooks},
    rooms::get_room_at,
};

struct PajamaSam(mpsc::UnboundedSender<tungstenite::Message>);

#[allow(unused)]
#[derive(Debug, Deserialize, JsonSchema)]
struct TestAction;

#[derive(Debug, Deserialize, JsonSchema)]
struct ClickObject {
    objectId: usize,
}

#[allow(unused)]
#[derive(Debug, Deserialize, JsonSchema)]
struct GetRoomId;

#[derive(Debug, neuro_sama::derive::Actions)]
enum Action {
    /// Get Room ID
    #[name = "get_room_id"]
    GetRoomId(GetRoomId),
    /// Clicks an object by ID
    #[name = "click_object"]
    ClickObject(ClickObject),
}

impl neuro_sama::game::Game for PajamaSam {
    const NAME: &'static str = "Pajama Sam";
    type Actions<'a> = Action;
    fn send_command(&self, message: tungstenite::Message) {
        let _ = self.0.send(message);
    }

    fn reregister_actions(&self) {
        return;
    }

    fn handle_action<'a>(
        &self,
        action: Self::Actions<'a>,
    ) -> Result<
        Option<impl 'static + Into<std::borrow::Cow<'static, str>>>,
        Option<impl 'static + Into<std::borrow::Cow<'static, str>>>,
    > {
        let engine = unsafe { ScummEngine::get().unwrap() };

        match action {
            Action::GetRoomId(_) => {
                Ok::<_, Option<String>>(Some(engine.get_current_room_id().to_string()))
            }
            Action::ClickObject(object) => {
                let room = get_room_at(engine.get_current_room_id())
                    .ok_or(Some("Room was invalid".to_string()))?;

                room.get_object(object.objectId as i32)
                    .ok_or(Some("Object was invalid".to_string()))?
                    .click(None)
                    .map(Some)
                    .map_err(|e| Some(e))
            }
        }
    }
}

#[derive(Debug)]
pub struct DialogLine {
    pub offset_id: u32,
    #[allow(unused)]
    pub length: u32,
    pub text: String,
}

pub static DIALOGUE_TX: OnceLock<UnboundedSender<DialogLine>> = OnceLock::new();

#[tokio::main(flavor = "current_thread")]
pub async fn init_game() {
    let (dialogue_tx, mut dialogue_rx) = tokio::sync::mpsc::unbounded_channel();
    DIALOGUE_TX
        .set(dialogue_tx)
        .expect("Failed to set DIALOGUE_TX");

    unsafe {
        init_hooks();
    }
    let (game2ws_tx, mut game2ws_rx) = mpsc::unbounded_channel();
    let game = Arc::new(PajamaSam(game2ws_tx));
    let init_res = game.initialize();

    if let Err(error) = init_res {
        println!("Failed to initialize neuro integration. {:?}", error);
        return;
    }

    let mut ws =
        match tokio_tungstenite::connect_async(if let Ok(url) = std::env::var("NEURO_SDK_WS_URL") {
            url
        } else {
            "ws://127.0.0.1:8000".to_owned()
        })
        .await
        {
            Ok(res) => res.0,
            Err(_) => {
                println!("Failed to connect to Neuro Server. Retrys are not implemented yet. :/");
                return;
            }
        };

    // is there a better way of doing this?
    // let game2 = game.clone();

    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(5)).await;
            // send_context_if_room_updated(&game2);
        }
    });

    loop {
        tokio::select! {
            msg = game2ws_rx.recv() => {
                // println!("game2ws {msg:?}");
                let Some(msg) = msg else {
                    break;
                };
                if ws.send(msg).await.is_err() {
                    println!("websocket send failed");
                    break;
                }
            }
            msg = ws.next() => {
                // println!("ws2game {msg:?}");
                let Some(msg) = msg else {
                    break;
                };
                let Ok(msg) = msg else {
                    continue;
                };
                if let Err(err) = game.handle_message(msg) {
                    // this could happen because we don't know what this message means (e.g. added
                    // in a new version of the API)
                    println!("notify_message failed: {err}");
                    continue;
                }
            }
            Some(dialogue) = dialogue_rx.recv() => {
                _ = game.context(
                    format!("{} just said: '{}'", dialogue.get_speaker(), dialogue.text),
                    false
                );
            }
        }
    }
}

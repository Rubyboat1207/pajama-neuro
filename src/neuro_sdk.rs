use std::{sync::{Arc, Mutex, OnceLock}, time::Duration};
use futures_util::{SinkExt, StreamExt};
use neuro_sama::game::Api;
use schemars::JsonSchema;
use serde::Deserialize;
use tokio::sync::{mpsc};

use crate::game::ScummEngine;

struct PajamaSam(mpsc::UnboundedSender<tungstenite::Message>);

#[allow(unused)]
#[derive(Debug, Deserialize, JsonSchema)]
struct TestAction;

#[derive(Debug, Deserialize, JsonSchema)]
struct ClickObject {
    objectId: usize
}

#[allow(unused)]
#[derive(Debug, Deserialize, JsonSchema)]
struct GetRoomId;

#[derive(Debug, neuro_sama::derive::Actions)]
enum Action {
    /// Action 1 description
    #[name = "action1"]
    TestAction(TestAction),
    /// Get Room ID
    #[name = "get_room_id"]
    GetRoomId(GetRoomId),
    /// Clicks an object by ID
    #[name = "click_object"]
    ClickObject(ClickObject)
}

static LAST_ROOM_ID: OnceLock<Arc<Mutex<i32>>> = OnceLock::new();

fn check_room_update() -> Option<i32> {
    let room_id = unsafe { ScummEngine::get().unwrap().get_current_room_id() };
    let data = LAST_ROOM_ID.get_or_init(|| Arc::new(Mutex::new(-1)));
    let mut value = data.lock().unwrap();
    
    if room_id != *value {
        *value = room_id;
        Some(room_id) // Return the new ID
    } else {
        None
    }
}

fn send_context_if_room_updated(game: &PajamaSam) {
    let _ = match check_room_update() {
        Some(new_id) => game.context(format!("You've moved into {}", get_room_name(new_id)), false),
        None => return,
    };
}

static ROOMS: &'static [&str] = &[
    "The Loading Screen",
    "The Intro",
    "Sam's Room"
];

fn get_room_name(id: i32) -> String {
    ROOMS[id as usize].to_string()
}

impl neuro_sama::game::Game for PajamaSam {
    const NAME: &'static str = "Pajama Sam";
    type Actions<'a> = Action;
    fn send_command(&self, message: tungstenite::Message) {
        let _ = self.0.send(message);
    }


    fn reregister_actions(&self) {
        self.register_actions::<Action>().unwrap();
    }


    fn handle_action<'a>(
        &self,
        action: Self::Actions<'a>,
    ) -> Result<
        Option<impl 'static + Into<std::borrow::Cow<'static, str>>>,
        Option<impl 'static + Into<std::borrow::Cow<'static, str>>>,
    > {
        let engine = unsafe { ScummEngine::get().unwrap() };
        send_context_if_room_updated(self);

        match action {
            Action::TestAction(_) => {
                let mut ids: Vec<u8> = vec![];
                let actor_count = unsafe { engine.get_num_actors() };

                println!("Saw {} actors", actor_count);

                for i in 0..actor_count {
                    match unsafe { engine.get_actor(i.into()) } {
                        Some(actor) => {
                            println!("Actor at {} found", i);
                            let id = unsafe { actor.get_id() };
                            println!("Actor at {} has id {}", i, id);
                            ids.push(id)
                        },
                        None => continue,
                    }
                }

                unsafe { engine.print_all_objects() };

                Ok(Some(format!("{:?}", ids)))
            },
            Action::GetRoomId(_) => Ok::<_, Option<String>>(Some(unsafe { engine.get_current_room_id() }.to_string())),
            Action::ClickObject(object) => {
                match unsafe { engine.get_room_object(object.objectId) } {
                    Some(obj) => {
                        unsafe { obj.click() };
                        Ok(Some("yay clicked!".to_string()))
                    },
                    None => Ok(Some("Wrong Object ID".to_string())),
                }
            }
        }
    }
}

#[tokio::main(flavor = "current_thread")]
pub async fn init_game() {
    let (game2ws_tx, mut game2ws_rx) = mpsc::unbounded_channel();
    let game = Arc::new(PajamaSam(game2ws_tx));
    game.initialize().unwrap();

    let mut ws =
        tokio_tungstenite::connect_async(if let Ok(url) = std::env::var("NEURO_SDK_WS_URL") {
            url
        } else {
            "ws://127.0.0.1:8000".to_owned()
        })
        .await
        .unwrap()
        .0;
    
    // is there a better way of doing this?
    let game2 = game.clone();

    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(5)).await;
            send_context_if_room_updated(&game2);
        }
    });

    loop {
        tokio::select! {
            msg = game2ws_rx.recv() => {
                println!("game2ws {msg:?}");
                let Some(msg) = msg else {
                    break;
                };
                if ws.send(msg).await.is_err() {
                    println!("websocket send failed");
                    break;
                }
            }
            msg = ws.next() => {
                println!("ws2game {msg:?}");
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

        }
    }
}
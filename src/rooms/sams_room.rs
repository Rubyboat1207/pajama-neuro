use crate::{
    game::engine::{PajamaSamGame, ScummEngine},
    rooms::{ObjectDescription, RoomDescription},
};

pub(crate) const SAMS_ROOM_DESCRIPTION: RoomDescription = RoomDescription {
    id: 2,
    name: "Sam's Room",
    on_entered: || "".to_string(),
    objects: &[
        ObjectDescription {
            id: 66,
            name: "Closet Door",
            on_clicked: || Ok("".to_string()),
            click_offset: None,
        },
        ObjectDescription {
            id: 67,
            name: "Under Sam's Bed",
            on_clicked: || if unsafe { ScummEngine::get().unwrap().get_scumm_var(478) } == 1 && !has_lunchbox() {
                Ok("Under your bed is your lunchbox!".to_string())
            } else { Ok("Nothing under here!".to_string())} ,
            click_offset: Some((-25, 0)),
        },
        ObjectDescription {
            id: 68,
            name: "Garbage Can",
            on_clicked: || if unsafe { ScummEngine::get().unwrap().get_scumm_var(478) } == 0 && !has_lunchbox() {
                Ok("Under some scraps of paper is your lunchbox!".to_string())
            } else { Ok("Nothing in here!".to_string())} ,
            click_offset: None,
        },
        ObjectDescription {
            id: 69,
            name: "Top Nightstand Drawer",
            on_clicked: || if unsafe { ScummEngine::get().unwrap().get_scumm_var(477) } == 1 && !has_flashlight() {
                Ok("You looked inside and here is your flashlight!".to_string())
            } else { Ok("Nothing in here!".to_string())} ,
            click_offset: None,
        },
        ObjectDescription {
            id: 70,
            name: "Bottom Nightstand Drawer",
            on_clicked: || if unsafe { ScummEngine::get().unwrap().get_scumm_var(477) } == 0 && !has_flashlight() {
                Ok("You looked inside and here is your flashlight!".to_string())
            } else { Ok("Nothing in here!".to_string())} ,
            click_offset: None,
        },
        ObjectDescription {
            id: 71,
            name: "The Rug",
            on_clicked: || if unsafe { ScummEngine::get().unwrap().get_scumm_var(476) } == 1 && !has_mask() {
                Ok("Under the rug, is your Pajama Sam Mask!".to_string())
            } else { Ok("Nothing under here!".to_string())} ,
            click_offset: None,
        },
        ObjectDescription {
            id: 72,
            name: "The Clothes Rack",
            on_clicked: || if unsafe { ScummEngine::get().unwrap().get_scumm_var(476) } == 0 && !has_mask() {
                Ok("On your clothes rack is your Pajama Sam Mask!".to_string())
            } else { Ok("Nothing on here!".to_string())} ,
            click_offset: None,
        },
        ObjectDescription {
            id: 73,
            name: "Door",
            on_clicked: || Ok("".to_string()),
            click_offset: None,
        },
        ObjectDescription {
            id: 74,
            name: "Pajama Sam",
            on_clicked: || Ok("".to_string()),
            click_offset: None,
        },
        ObjectDescription {
            id: 75,
            name: "Top left bed Knob",
            on_clicked: || Ok("".to_string()),
            click_offset: None,
        },
        ObjectDescription {
            id: 76,
            name: "Top right bed Knob",
            on_clicked: || Ok("".to_string()),
            click_offset: None,
        },
        ObjectDescription {
            id: 77,
            name: "Bottom right Bed Knob",
            on_clicked: || Ok("".to_string()),
            click_offset: None,
        },
        ObjectDescription {
            id: 78,
            name: "Bottom Left Bed Knob",
            on_clicked: || Ok("".to_string()),
            click_offset: None,
        },
        ObjectDescription {
            id: 79,
            name: "Books On Shelf",
            on_clicked: || Ok("".to_string()),
            click_offset: None,
        },
        ObjectDescription {
            id: 80,
            name: "Paper Nailed To Door",
            on_clicked: || Ok("".to_string()),
            click_offset: None,
        },
        ObjectDescription {
            id: 81,
            name: "Poster",
            on_clicked: || Ok("".to_string()),
            click_offset: None,
        },
        ObjectDescription {
            id: 82,
            name: "Ceiling Light Fixture",
            on_clicked: || Ok("".to_string()),
            click_offset: None,
        },
        ObjectDescription {
            id: 83,
            name: "Wall Poster (Pajama Man)",
            on_clicked: || Ok("".to_string()),
            click_offset: None,
        },
        ObjectDescription {
            id: 84,
            name: "Toy Airplane",
            on_clicked: || Ok("".to_string()),
            click_offset: None,
        },
        ObjectDescription {
            id: 85,
            name: "SAM ST Street Sign",
            on_clicked: || Ok("".to_string()),
            click_offset: None,
        },
        ObjectDescription {
            id: 86,
            name: "Pajama Man Night Light",
            on_clicked: || Ok("".to_string()),
            click_offset: None,
        },
        ObjectDescription {
            id: 87,
            name: "Hanging Baseball Glove",
            on_clicked: || Ok("".to_string()),
            click_offset: None,
        },
        ObjectDescription {
            id: 88,
            name: "Hanging Baseball Cap",
            on_clicked: || Ok("".to_string()),
            click_offset: None,
        },
        ObjectDescription {
            id: 89,
            name: "Lamp Shade",
            on_clicked: || Ok("".to_string()),
            click_offset: None,
        },
        ObjectDescription {
            id: 90,
            name: "Calendar",
            on_clicked: || Ok("".to_string()),
            click_offset: None,
        },
        ObjectDescription {
            id: 91,
            name: "Toy Truck",
            on_clicked: || Ok("".to_string()),
            click_offset: None,
        },
        ObjectDescription {
            id: 93,
            name: "On top of the bed",
            on_clicked: || Ok("".to_string()),
            click_offset: None,
        },
        ObjectDescription {
            id: 94,
            name: "Pillow",
            on_clicked: || Ok("".to_string()),
            click_offset: None,
        },
        ObjectDescription {
            id: 95,
            name: "Comic Book",
            on_clicked: || Ok("".to_string()),
            click_offset: None,
        },
        ObjectDescription {
            id: 97,
            name: "Closet Door (Left)",
            on_clicked: || Ok("".to_string()),
            click_offset: None,
        },
    ],
};


pub fn has_flashlight() -> bool {
    unsafe { ScummEngine::get().unwrap().has_item(18) }
}

fn has_lunchbox() -> bool {
    unsafe { ScummEngine::get().unwrap().has_item(19) }
}

fn has_mask() -> bool {
    unsafe { ScummEngine::get().unwrap().has_item(20) }
}
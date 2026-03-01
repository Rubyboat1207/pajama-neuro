use crate::{
    game::engine::{PajamaSamGame, ScummEngine},
    rooms::{ObjectDescription, RoomDescription},
};

pub(crate) const SAMS_ROOM_DESCRIPTION: RoomDescription = RoomDescription {
    silent: false,
    id: 2,
    name: "Sam's Room",
    on_entered: || "".to_string(),
    objects: &[
        ObjectDescription::new(66, "Closet Door", || {
            if !has_flashlight() && !has_lunchbox() && !has_mask() {
                Ok(
                    "You can't enter the closet without your flashlight, lunchbox, and mask!"
                        .to_string(),
                )
            } else {
                Ok("You enter the closet.".to_string())
            }
        }),
        ObjectDescription::new_with_offset(
            67,
            "Under Sam's Bed",
            || {
                if unsafe { ScummEngine::get().unwrap().get_scumm_var(478) } == 1 && !has_lunchbox()
                {
                    Ok("Under your bed is your lunchbox!".to_string())
                } else {
                    Ok("Nothing under here!".to_string())
                }
            },
            Some((-25, 0)),
        ),
        ObjectDescription::new(68, "Garbage Can", || {
            if unsafe { ScummEngine::get().unwrap().get_scumm_var(478) } == 0 && !has_lunchbox() {
                Ok("Under some scraps of paper is your lunchbox!".to_string())
            } else {
                Ok("Nothing in here!".to_string())
            }
        }),
        ObjectDescription::new(69, "Top Nightstand Drawer", || {
            if unsafe { ScummEngine::get().unwrap().get_scumm_var(477) } == 1
                    && !has_flashlight()
                {
                    Ok("You looked inside and here is your flashlight!".to_string())
                } else {
                    Ok("Nothing in here!".to_string())
                }
            }
        ),
        ObjectDescription::new(70, "Bottom Nightstand Drawer", || {
            if unsafe { ScummEngine::get().unwrap().get_scumm_var(477) } == 0
                    && !has_flashlight()
                {
                    Ok("You looked inside and here is your flashlight!".to_string())
                } else {
                    Ok("Nothing in here!".to_string())
                }
            }
        ),
        ObjectDescription::new(71, "The Rug", || {
            if unsafe { ScummEngine::get().unwrap().get_scumm_var(476) } == 1 && !has_mask() {
                    Ok("Under the rug, is your Pajama Sam Mask!".to_string())
                } else {
                    Ok("Nothing under here!".to_string())
                }
            }
        ),
        ObjectDescription::new(72, "The Clothes Rack", || {
            if unsafe { ScummEngine::get().unwrap().get_scumm_var(476) } == 0 && !has_mask() {
                    Ok("On your clothes rack is your Pajama Sam Mask!".to_string())
                } else {
                    Ok("Nothing on here!".to_string())
                }
            }
        ),
        ObjectDescription::new(73, "Door", || Ok("You knock on the door...".to_string())),
        ObjectDescription::static_response(74, "Pajama Sam", "You talk to Pajama Sam..."),
        ObjectDescription::placeholder(75, "Top Left bed Knob"),
        ObjectDescription::placeholder(76, "Top right bed Knob"),
        ObjectDescription::placeholder(77, "Bottom right Bed Knob"),
        ObjectDescription::placeholder(78, "Bottom Left Bed Knob"),
        ObjectDescription::placeholder(79, "Books On Shelf"),
        ObjectDescription::placeholder(80, "Paper Nailed To Door"),
        ObjectDescription::placeholder(81, "Poster"),
        ObjectDescription::placeholder(82, "Ceiling Light Fixture"),
        ObjectDescription::placeholder(83, "Wall Poster (Pajama Man)"),
        ObjectDescription::placeholder(84, "Toy Airplane"),
        ObjectDescription::placeholder(85, "SAM ST Street Sign"),
        ObjectDescription::placeholder(86, "Pajama Man Night Light"),
        ObjectDescription::placeholder(87, "Hanging Baseball Glove"),
        ObjectDescription::placeholder(88, "Hanging Baseball Cap"),
        ObjectDescription::placeholder(89, "Lamp Shade"),
        ObjectDescription::placeholder(90, "Calendar"),
        ObjectDescription::placeholder(91, "Toy Truck"),
        ObjectDescription::placeholder(93, "On top of the bed"),
        ObjectDescription::placeholder(94, "Pillow"),
        ObjectDescription::placeholder(95, "Comic Book"),
        ObjectDescription::placeholder(96, "Closet Door (Left)")
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

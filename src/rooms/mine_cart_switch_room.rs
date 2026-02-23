use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const MINE_CART_SWITCH_ROOM_DESCRIPTION: RoomDescription = RoomDescription {
    id: 32,
    name: "Mine Cart Switch Room",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

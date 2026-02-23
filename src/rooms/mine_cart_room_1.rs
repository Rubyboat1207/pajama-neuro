use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const MINE_CART_ROOM_1_DESCRIPTION: RoomDescription = RoomDescription {
    id: 25,
    name: "Mine Cart Room 1",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const MINE_CART_ROOM_2_DESCRIPTION: RoomDescription = RoomDescription {
    id: 26,
    name: "Mine Cart Room 2",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

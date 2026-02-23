use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const MINE_CART_JUMPING_ROOM_DESCRIPTION: RoomDescription = RoomDescription {
    id: 30,
    name: "Mine cart jumping room",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

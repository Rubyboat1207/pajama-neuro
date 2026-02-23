use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const MINE_CART_LAVA_ROOM_DESCRIPTION: RoomDescription = RoomDescription {
    id: 33,
    name: "Mine Cart Lava Room",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const DARK_MINE_CART_ROOM_DESCRIPTION: RoomDescription = RoomDescription {
    id: 29,
    name: "Dark Mine Cart Room",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

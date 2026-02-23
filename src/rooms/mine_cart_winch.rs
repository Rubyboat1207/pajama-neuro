use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const MINE_CART_WINCH_DESCRIPTION: RoomDescription = RoomDescription {
    id: 31,
    name: "Mine Cart Winch",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

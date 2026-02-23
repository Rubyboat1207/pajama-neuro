use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const Y_INTERSECTION_MINE_CART_ROOM_DESCRIPTION: RoomDescription = RoomDescription {
    id: 27,
    name: "Y Intersection Mine Cart Room",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

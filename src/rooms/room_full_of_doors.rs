use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const ROOM_FULL_OF_DOORS_DESCRIPTION: RoomDescription = RoomDescription {
    id: 47,
    name: "Room full of Doors",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const ROOM_FULL_OF_DOORS_2_DESCRIPTION: RoomDescription = RoomDescription {
    id: 48,
    name: "Room full of Doors 2",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

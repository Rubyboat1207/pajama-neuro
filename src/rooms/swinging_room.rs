use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const SWINGING_ROOM_DESCRIPTION: RoomDescription = RoomDescription {
    id: 51,
    name: "Swinging Room",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

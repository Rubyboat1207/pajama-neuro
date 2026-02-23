use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const LIVING_ROOM_DESCRIPTION: RoomDescription = RoomDescription {
    id: 44,
    name: "Living Room",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const GRANDFATHER_CLOCK_ROOM_DESCRIPTION: RoomDescription = RoomDescription {
    id: 49,
    name: "Grandfather Clock Room",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

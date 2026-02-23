use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const STAIR_ROOM_DESCRIPTION: RoomDescription = RoomDescription {
    id: 53,
    name: "Stair Room",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

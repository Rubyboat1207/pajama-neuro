use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const DOOR_ROOM_DESCRIPTION: RoomDescription = RoomDescription {
    id: 45,
    name: "Door Room",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

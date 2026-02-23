use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const DARKNESS_ROOM_DESCRIPTION: RoomDescription = RoomDescription {
    id: 57,
    name: "Darkness' Room",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

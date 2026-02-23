use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const MUSIC_ROOM_DESCRIPTION: RoomDescription = RoomDescription {
    id: 50,
    name: "Music Room",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

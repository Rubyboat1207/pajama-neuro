use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const DARKNESS_DOOR_DESCRIPTION: RoomDescription = RoomDescription {
    id: 46,
    name: "Darkness' Door",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

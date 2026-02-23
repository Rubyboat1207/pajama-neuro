use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const BACK_DOOR_DESCRIPTION: RoomDescription = RoomDescription {
    id: 34,
    name: "Back Door",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

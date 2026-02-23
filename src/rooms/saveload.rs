use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const SAVELOAD_DESCRIPTION: RoomDescription = RoomDescription {
    id: 68,
    name: "SaveLoad",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

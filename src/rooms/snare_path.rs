use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const SNARE_PATH_DESCRIPTION: RoomDescription = RoomDescription {
    id: 5,
    name: "Snare Path",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

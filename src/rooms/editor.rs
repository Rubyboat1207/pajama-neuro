use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const EDITOR_DESCRIPTION: RoomDescription = RoomDescription {
    id: 62,
    name: "editor",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

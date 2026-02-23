use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const INNER_CAVERN_DESCRIPTION: RoomDescription = RoomDescription {
    id: 19,
    name: "Inner Cavern",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

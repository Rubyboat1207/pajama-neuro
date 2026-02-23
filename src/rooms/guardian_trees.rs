use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const GUARDIAN_TREES_DESCRIPTION: RoomDescription = RoomDescription {
    id: 11,
    name: "Guardian Trees",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

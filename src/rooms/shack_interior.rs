use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const SHACK_INTERIOR_DESCRIPTION: RoomDescription = RoomDescription {
    id: 16,
    name: "Shack Interior",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

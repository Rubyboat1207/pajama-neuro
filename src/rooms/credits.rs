use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const CREDITS_DESCRIPTION: RoomDescription = RoomDescription {
    id: 66,
    name: "Credits",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

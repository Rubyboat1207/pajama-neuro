use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const THE_SHACK_DESCRIPTION: RoomDescription = RoomDescription {
    id: 15,
    name: "The Shack",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

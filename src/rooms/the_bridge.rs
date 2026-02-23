use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const THE_BRIDGE_DESCRIPTION: RoomDescription = RoomDescription {
    id: 4,
    name: "The bridge",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const THE_DOCK_DESCRIPTION: RoomDescription = RoomDescription {
    id: 7,
    name: "The dock",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

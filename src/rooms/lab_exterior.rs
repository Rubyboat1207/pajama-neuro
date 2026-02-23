use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const LAB_EXTERIOR_DESCRIPTION: RoomDescription = RoomDescription {
    id: 54,
    name: "Lab Exterior",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

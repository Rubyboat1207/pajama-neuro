use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const LAB_DESCRIPTION: RoomDescription = RoomDescription {
    id: 55,
    name: "Lab",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

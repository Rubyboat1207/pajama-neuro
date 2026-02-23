use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const JUNCTION_DESCRIPTION: RoomDescription = RoomDescription {
    id: 6,
    name: "Junction",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

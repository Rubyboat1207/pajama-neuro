use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const GARDEN_DESCRIPTION: RoomDescription = RoomDescription {
    id: 12,
    name: "Garden",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

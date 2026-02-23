use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const POTION_TABLE_DESCRIPTION: RoomDescription = RoomDescription {
    id: 56,
    name: "Potion Table",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

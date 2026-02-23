use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const INVENTORY_DESCRIPTION: RoomDescription = RoomDescription {
    id: 60,
    name: "inventory",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

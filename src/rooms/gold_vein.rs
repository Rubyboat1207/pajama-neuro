use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const GOLD_VEIN_DESCRIPTION: RoomDescription = RoomDescription {
    id: 37,
    name: "Gold Vein",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

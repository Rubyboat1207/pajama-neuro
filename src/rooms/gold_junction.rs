use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const GOLD_JUNCTION_DESCRIPTION: RoomDescription = RoomDescription {
    id: 35,
    name: "Gold Junction",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

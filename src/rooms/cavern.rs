use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const CAVERN_DESCRIPTION: RoomDescription = RoomDescription {
    id: 18,
    name: "Cavern",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

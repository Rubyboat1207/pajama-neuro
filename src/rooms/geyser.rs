use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const GEYSER_DESCRIPTION: RoomDescription = RoomDescription {
    id: 22,
    name: "Geyser",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

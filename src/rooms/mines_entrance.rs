use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const MINES_ENTRANCE_DESCRIPTION: RoomDescription = RoomDescription {
    id: 24,
    name: "Mines Entrance",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

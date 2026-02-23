use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const THE_STONE_BRIDGE_ENTRANCE_DESCRIPTION: RoomDescription = RoomDescription {
    id: 13,
    name: "The stone bridge Entrance",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

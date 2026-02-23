use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const WILD_GOLD_ROOM_DESCRIPTION: RoomDescription = RoomDescription {
    id: 36,
    name: "Wild Gold Room",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

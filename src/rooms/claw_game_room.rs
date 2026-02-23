use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const CLAW_GAME_ROOM_DESCRIPTION: RoomDescription = RoomDescription {
    id: 38,
    name: "Claw Game Room",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

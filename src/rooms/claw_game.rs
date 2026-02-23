use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const CLAW_GAME_DESCRIPTION: RoomDescription = RoomDescription {
    id: 39,
    name: "Claw Game",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

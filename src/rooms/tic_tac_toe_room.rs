use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const TIC_TAC_TOE_ROOM_DESCRIPTION: RoomDescription = RoomDescription {
    id: 14,
    name: "Tic Tac Toe Room",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

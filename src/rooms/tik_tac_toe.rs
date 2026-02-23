use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const TIK_TAC_TOE_DESCRIPTION: RoomDescription = RoomDescription {
    id: 64,
    name: "Tik Tac Toe",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

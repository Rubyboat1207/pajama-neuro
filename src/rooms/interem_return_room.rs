use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const INTEREM_RETURN_ROOM_DESCRIPTION: RoomDescription = RoomDescription {
    id: 40,
    name: "Interem Return Room",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

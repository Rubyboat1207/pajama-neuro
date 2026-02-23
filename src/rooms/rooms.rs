use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const ROOMS_DESCRIPTION: RoomDescription = RoomDescription {
    id: 59,
    name: "rooms",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const DARKNESS_CLOSET_DESCRIPTION: RoomDescription = RoomDescription {
    id: 58,
    name: "Darkness' Closet",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

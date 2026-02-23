use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const CONNECT_THE_DOTS_WALL_DESCRIPTION: RoomDescription = RoomDescription {
    id: 28,
    name: "Connect the dots Wall",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

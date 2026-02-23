use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const THE_ENTRANCE_TO_THE_DARK_WORLD_DESCRIPTION: RoomDescription = RoomDescription {
    id: 3,
    name: "The Entrance to the dark world",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const FOREST_PATH_DESCRIPTION: RoomDescription = RoomDescription {
    id: 10,
    name: "Forest Path",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

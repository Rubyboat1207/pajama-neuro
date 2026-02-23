use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const THE_RAPIDS_DESCRIPTION: RoomDescription = RoomDescription {
    id: 17,
    name: "The Rapids",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

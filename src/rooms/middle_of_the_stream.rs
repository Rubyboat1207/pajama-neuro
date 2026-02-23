use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const MIDDLE_OF_THE_STREAM_DESCRIPTION: RoomDescription = RoomDescription {
    id: 8,
    name: "Middle of the stream",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

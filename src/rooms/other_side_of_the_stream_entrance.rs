use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const OTHER_SIDE_OF_THE_STREAM_ENTRANCE_DESCRIPTION: RoomDescription = RoomDescription {
    id: 9,
    name: "Other side of the stream entrance",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

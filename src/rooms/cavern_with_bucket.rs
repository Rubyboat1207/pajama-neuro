use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const CAVERN_WITH_BUCKET_DESCRIPTION: RoomDescription = RoomDescription {
    id: 21,
    name: "Cavern with bucket",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

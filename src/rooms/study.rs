use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const STUDY_DESCRIPTION: RoomDescription = RoomDescription {
    id: 52,
    name: "Study",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

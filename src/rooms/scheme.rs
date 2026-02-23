use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const SCHEME_DESCRIPTION: RoomDescription = RoomDescription {
    id: 61,
    name: "scheme",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const IG_LOGO_DESCRIPTION: RoomDescription = RoomDescription {
    id: 67,
    name: "IG Logo",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

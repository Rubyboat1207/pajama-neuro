use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const HE_LOGO_DESCRIPTION: RoomDescription = RoomDescription {
    id: 1,
    name: "HE Logo",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

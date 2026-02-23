use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const BY_THE_WELL_DESCRIPTION: RoomDescription = RoomDescription {
    id: 20,
    name: "By the Well",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

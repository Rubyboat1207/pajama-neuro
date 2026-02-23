use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const KITCHEN_DESCRIPTION: RoomDescription = RoomDescription {
    id: 43,
    name: "Kitchen",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

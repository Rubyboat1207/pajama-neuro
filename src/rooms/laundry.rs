use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const LAUNDRY_DESCRIPTION: RoomDescription = RoomDescription {
    id: 65,
    name: "Laundry",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const GRANDFATHER_CLOCK_GEARS_DESCRIPTION: RoomDescription = RoomDescription {
    id: 63,
    name: "Grandfather Clock Gears",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

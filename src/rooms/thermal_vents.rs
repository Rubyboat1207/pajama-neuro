use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const THERMAL_VENTS_DESCRIPTION: RoomDescription = RoomDescription {
    id: 23,
    name: "Thermal Vents",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

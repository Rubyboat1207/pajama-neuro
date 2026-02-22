use crate::rooms::{RoomDescription};

pub(crate) const INTRO_DESCRIPTION: RoomDescription = RoomDescription {
    id: 67,
    name: "The intro cut-scene",
    on_entered: || "".to_string(),
    objects: &[]
};

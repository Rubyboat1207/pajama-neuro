use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const MAIN_TREE_HOUSE_HALL_DESCRIPTION: RoomDescription = RoomDescription {
    id: 42,
    name: "Main Tree House Hall",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

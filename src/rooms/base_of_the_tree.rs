use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const BASE_OF_THE_TREE_DESCRIPTION: RoomDescription = RoomDescription {
    id: 41,
    name: "Base of the tree",
    on_entered: || "".to_string(),
    objects: &[
        // Add objects here
    ],
};

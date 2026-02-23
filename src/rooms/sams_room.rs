use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const SAMS_ROOM_DESCRIPTION: RoomDescription = RoomDescription {
    id: 2,
    name: "Sam's Room",
    on_entered: || "".to_string(),
    objects: &[
        ObjectDescription {
            id: 66,
            name: "Closet Door",
            on_clicked: || Ok("".to_string()),
            click_offset: None,
        },
        ObjectDescription {
            id: 67,
            name: "Under Sam's Bed",
            on_clicked: || Ok("".to_string()),
            click_offset: Some((-25, 0))
        },
        ObjectDescription {
            id: 68,
            name: "Garbage Can",
            on_clicked: || Ok("".to_string()),
            click_offset: None
        },
        ObjectDescription {
            id: 69,
            name: "Top Nightstand Drawer",
            on_clicked: || Ok("".to_string()),
            click_offset: None
        },
        ObjectDescription {
            id: 70,
            name: "Bottom Nightstand Drawer",
            on_clicked: || Ok("".to_string()),
            click_offset: None
        },
        ObjectDescription {
            id: 72,
            name: "Purple Scarf / Bedpost",
            on_clicked: || Ok("".to_string()),
            click_offset: None
        },
        ObjectDescription {
            id: 73,
            name: "Door",
            on_clicked: || Ok("".to_string()),
            click_offset: None
        },
        ObjectDescription {
            id: 74,
            name: "Pajama Sam",
            on_clicked: || Ok("".to_string()),
            click_offset: None
        },
        ObjectDescription {
            id: 75,
            name: "Top left bed Knob",
            on_clicked: || Ok("".to_string()),
            click_offset: None
        },
        ObjectDescription {
            id: 76,
            name: "Top right bed Knob",
            on_clicked: || Ok("".to_string()),
            click_offset: None
        },
        ObjectDescription {
            id: 77,
            name: "Bottom right Bed Knob",
            on_clicked: || Ok("".to_string()),
            click_offset: None
        },
        ObjectDescription {
            id: 78,
            name: "Bottom Left Bed Knob",
            on_clicked: || Ok("".to_string()),
            click_offset: None
        },
        ObjectDescription {
            id: 79,
            name: "Books On Shelf",
            on_clicked: || Ok("".to_string()),
            click_offset: None
        },
        ObjectDescription {
            id: 80,
            name: "Paper Nailed To Door",
            on_clicked: || Ok("".to_string()),
            click_offset: None
        },
        ObjectDescription {
            id: 81,
            name: "Poster",
            on_clicked: || Ok("".to_string()),
            click_offset: None
        },
        ObjectDescription {
            id: 82,
            name: "Ceiling Light Fixture",
            on_clicked: || Ok("".to_string()),
            click_offset: None
        },
        ObjectDescription {
            id: 83,
            name: "Wall Poster (Pajama Man)",
            on_clicked: || Ok("".to_string()),
            click_offset: None
        },
        ObjectDescription {
            id: 84,
            name: "Toy Airplane",
            on_clicked: || Ok("".to_string()),
            click_offset: None
        },
        ObjectDescription {
            id: 85,
            name: "SAM ST Street Sign",
            on_clicked: || Ok("".to_string()),
            click_offset: None
        },
        ObjectDescription {
            id: 86,
            name: "Pajama Man Night Light",
            on_clicked: || Ok("".to_string()),
            click_offset: None
        },
        ObjectDescription {
            id: 87,
            name: "Hanging Baseball Glove",
            on_clicked: || Ok("".to_string()),
            click_offset: None
        },
        ObjectDescription {
            id: 88,
            name: "Hanging Baseball Cap",
            on_clicked: || Ok("".to_string()),
            click_offset: None
        },
        ObjectDescription {
            id: 89,
            name: "Lamp Shade",
            on_clicked: || Ok("".to_string()),
            click_offset: None
        },
        ObjectDescription {
            id: 90,
            name: "Calendar",
            on_clicked: || Ok("".to_string()),
            click_offset: None
        },
        ObjectDescription {
            id: 91,
            name: "Toy Truck",
            on_clicked: || Ok("".to_string()),
            click_offset: None
        },
        ObjectDescription {
            id: 93,
            name: "On top of the bed",
            on_clicked: || Ok("".to_string()),
            click_offset: None
        },
        ObjectDescription {
            id: 94,
            name: "Pillow",
            on_clicked: || Ok("".to_string()),
            click_offset: None
        },
        ObjectDescription {
            id: 95,
            name: "Comic Book",
            on_clicked: || Ok("".to_string()),
            click_offset: None
        },
        ObjectDescription {
            id: 97,
            name: "Closet Door (Left)",
            on_clicked: || Ok("".to_string()),
            click_offset: None
        },
    ],
};
use crate::rooms::{ObjectDescription, RoomDescription};

pub(crate) const BEDROOM_DESCRIPTION: RoomDescription = RoomDescription {
    id: 2,
    name: "Sam's Bedroom",
    on_entered: || "".to_string(),
    objects: &[
        ObjectDescription {
            id: 66,
            name: "Closet Door",
            on_clicked: || Ok("".to_string()),
        },
        ObjectDescription {
            id: 67,
            name: "Under Sam's Bed",
            on_clicked: || Ok("".to_string()),
        },
        ObjectDescription {
            id: 68,
            name: "Pot",
            on_clicked: || Ok("".to_string()),
        },
        ObjectDescription {
            id: 69,
            name: "Top Nightstand Drawer",
            on_clicked: || Ok("".to_string()),
        },
        ObjectDescription {
            id: 70,
            name: "Bottom Nightstand Drawer",
            on_clicked: || Ok("".to_string()),
        },
        ObjectDescription {
            id: 72,
            name: "Purple Scarf / Bedpost",
            on_clicked: || Ok("".to_string()),
        },
        ObjectDescription {
            id: 73,
            name: "Door",
            on_clicked: || Ok("".to_string()),
        },
        ObjectDescription {
            id: 74,
            name: "Pajama Sam",
            on_clicked: || Ok("".to_string()),
        },
        ObjectDescription {
            id: 75,
            name: "Top left bed Knob",
            on_clicked: || Ok("".to_string()),
        },
        ObjectDescription {
            id: 76,
            name: "Top right bed Knob",
            on_clicked: || Ok("".to_string()),
        },
        ObjectDescription {
            id: 77,
            name: "Bottom right Bed Knob",
            on_clicked: || Ok("".to_string()),
        },
        ObjectDescription {
            id: 78,
            name: "Bottom Left Bed Knob",
            on_clicked: || Ok("".to_string()),
        },
        ObjectDescription {
            id: 79,
            name: "Books On Shelf",
            on_clicked: || Ok("".to_string()),
        },
        ObjectDescription {
            id: 80,
            name: "Paper Nailed To Door",
            on_clicked: || Ok("".to_string()),
        },
        ObjectDescription {
            id: 81,
            name: "Poster",
            on_clicked: || Ok("".to_string()),
        },
        ObjectDescription {
            id: 82,
            name: "Ceiling Light Fixture",
            on_clicked: || Ok("".to_string()),
        },
        ObjectDescription {
            id: 83,
            name: "Wall Poster (Pajama Man)",
            on_clicked: || Ok("".to_string()),
        },
        ObjectDescription {
            id: 84,
            name: "Toy Airplane",
            on_clicked: || Ok("".to_string()),
        },
        ObjectDescription {
            id: 85,
            name: "SAM ST Street Sign",
            on_clicked: || Ok("".to_string()),
        },
        ObjectDescription {
            id: 86,
            name: "Pajama Man Night Light",
            on_clicked: || Ok("".to_string()),
        },
        ObjectDescription {
            id: 87,
            name: "Hanging Baseball Glove",
            on_clicked: || Ok("".to_string()),
        },
        ObjectDescription {
            id: 88,
            name: "Hanging Baseball Cap",
            on_clicked: || Ok("".to_string()),
        },
        ObjectDescription {
            id: 89,
            name: "Lamp Shade",
            on_clicked: || Ok("".to_string()),
        },
        ObjectDescription {
            id: 90,
            name: "Calendar",
            on_clicked: || Ok("".to_string()),
        },
        ObjectDescription {
            id: 91,
            name: "Toy Truck",
            on_clicked: || Ok("".to_string()),
        },
        ObjectDescription {
            id: 93,
            name: "On top of the bed",
            on_clicked: || Ok("".to_string()),
        },
        ObjectDescription {
            id: 94,
            name: "Pillow",
            on_clicked: || Ok("".to_string()),
        },
        ObjectDescription {
            id: 95,
            name: "Comic Book",
            on_clicked: || Ok("".to_string()),
        },
        ObjectDescription {
            id: 97,
            name: "Closet Door (Left)",
            on_clicked: || Ok("".to_string()),
        },
    ],
};

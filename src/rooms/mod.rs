use crate::rooms::{bedroom::BEDROOM_DESCRIPTION, intro::INTRO_DESCRIPTION};

#[derive(Debug)]
pub struct ObjectDescription {
    pub id: i32,
    pub name: &'static str,
    pub on_clicked: fn() -> Result<String, String>,
    pub click_offset: Option<(i32, i32)>
}

#[derive(Debug)]
pub struct RoomDescription {
    pub id: i32,
    pub name: &'static str,
    pub on_entered: fn() -> String,
    pub objects: &'static [ObjectDescription]
}

impl RoomDescription {
    pub fn get_object(&self, id: i32) -> Option<&'static ObjectDescription> {
        self.objects.iter().find(|item| item.id == id)
    }
}

mod bedroom;
mod intro;

pub const ROOMS: &'static [RoomDescription] = &[
    INTRO_DESCRIPTION,
    BEDROOM_DESCRIPTION
];

pub fn get_room_at(id: i32) -> Option<&'static RoomDescription> {
    ROOMS.iter().find(|item| item.id == id)
}
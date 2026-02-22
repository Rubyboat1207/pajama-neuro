use crate::rooms::bedroom::BEDROOM_DESCRIPTION;

pub struct ObjectDescription {
    pub id: i32,
    pub name: &'static str,
    pub on_clicked: fn() -> Result<String, String>
}

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

pub const ROOMS: &'static [RoomDescription] = &[
    BEDROOM_DESCRIPTION
];

pub fn get_room_at(id: i32) -> Option<&'static RoomDescription> {
    ROOMS.iter().find(|item| item.id == id)
}
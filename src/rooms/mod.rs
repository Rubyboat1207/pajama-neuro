#![allow(warnings)]
use crate::rooms::{
    back_door::BACK_DOOR_DESCRIPTION,
    base_of_the_tree::BASE_OF_THE_TREE_DESCRIPTION,
    by_the_well::BY_THE_WELL_DESCRIPTION,
    cavern::CAVERN_DESCRIPTION,
    cavern_with_bucket::CAVERN_WITH_BUCKET_DESCRIPTION,
    claw_game::CLAW_GAME_DESCRIPTION,
    claw_game_room::CLAW_GAME_ROOM_DESCRIPTION,
    connect_the_dots_wall::CONNECT_THE_DOTS_WALL_DESCRIPTION,
    credits::CREDITS_DESCRIPTION,
    dark_mine_cart_room::DARK_MINE_CART_ROOM_DESCRIPTION,
    darkness_closet::DARKNESS_CLOSET_DESCRIPTION,
    darkness_door::DARKNESS_DOOR_DESCRIPTION,
    darkness_room::DARKNESS_ROOM_DESCRIPTION,
    door_room::DOOR_ROOM_DESCRIPTION,
    editor::EDITOR_DESCRIPTION,
    forest_path::FOREST_PATH_DESCRIPTION,
    garden::GARDEN_DESCRIPTION,
    geyser::GEYSER_DESCRIPTION,
    gold_junction::GOLD_JUNCTION_DESCRIPTION,
    gold_vein::GOLD_VEIN_DESCRIPTION,
    grandfather_clock_gears::GRANDFATHER_CLOCK_GEARS_DESCRIPTION,
    grandfather_clock_room::GRANDFATHER_CLOCK_ROOM_DESCRIPTION,
    guardian_trees::GUARDIAN_TREES_DESCRIPTION,
    he_logo::HE_LOGO_DESCRIPTION,
    ig_logo::IG_LOGO_DESCRIPTION,
    inner_cavern::INNER_CAVERN_DESCRIPTION,
    interem_return_room::INTEREM_RETURN_ROOM_DESCRIPTION,
    inventory::INVENTORY_DESCRIPTION,
    junction::JUNCTION_DESCRIPTION,
    kitchen::KITCHEN_DESCRIPTION,
    lab::LAB_DESCRIPTION,
    lab_exterior::LAB_EXTERIOR_DESCRIPTION,
    laundry::LAUNDRY_DESCRIPTION,
    living_room::LIVING_ROOM_DESCRIPTION,
    main_tree_house_hall::MAIN_TREE_HOUSE_HALL_DESCRIPTION,
    middle_of_the_stream::MIDDLE_OF_THE_STREAM_DESCRIPTION,
    mine_cart_jumping_room::MINE_CART_JUMPING_ROOM_DESCRIPTION,
    mine_cart_lava_room::MINE_CART_LAVA_ROOM_DESCRIPTION,
    mine_cart_room_1::MINE_CART_ROOM_1_DESCRIPTION,
    mine_cart_room_2::MINE_CART_ROOM_2_DESCRIPTION,
    mine_cart_switch_room::MINE_CART_SWITCH_ROOM_DESCRIPTION,
    mine_cart_winch::MINE_CART_WINCH_DESCRIPTION,
    mines_entrance::MINES_ENTRANCE_DESCRIPTION,
    music_room::MUSIC_ROOM_DESCRIPTION,
    other_side_of_the_stream_entrance::OTHER_SIDE_OF_THE_STREAM_ENTRANCE_DESCRIPTION,
    potion_table::POTION_TABLE_DESCRIPTION,
    room_full_of_doors::ROOM_FULL_OF_DOORS_DESCRIPTION,
    room_full_of_doors_2::ROOM_FULL_OF_DOORS_2_DESCRIPTION,
    rooms::ROOMS_DESCRIPTION,
    sams_room::SAMS_ROOM_DESCRIPTION,
    saveload::SAVELOAD_DESCRIPTION,
    scheme::SCHEME_DESCRIPTION,
    shack_interior::SHACK_INTERIOR_DESCRIPTION,
    snare_path::SNARE_PATH_DESCRIPTION,
    stair_room::STAIR_ROOM_DESCRIPTION,
    study::STUDY_DESCRIPTION,
    swinging_room::SWINGING_ROOM_DESCRIPTION,
    the_bridge::THE_BRIDGE_DESCRIPTION,
    the_dock::THE_DOCK_DESCRIPTION,
    the_entrance_to_the_dark_world::THE_ENTRANCE_TO_THE_DARK_WORLD_DESCRIPTION,
    the_rapids::THE_RAPIDS_DESCRIPTION,
    the_shack::THE_SHACK_DESCRIPTION,
    the_stone_bridge_entrance::THE_STONE_BRIDGE_ENTRANCE_DESCRIPTION,
    thermal_vents::THERMAL_VENTS_DESCRIPTION,
    tic_tac_toe_room::TIC_TAC_TOE_ROOM_DESCRIPTION,
    tik_tac_toe::TIK_TAC_TOE_DESCRIPTION,
    wild_gold_room::WILD_GOLD_ROOM_DESCRIPTION,
    y_intersection_mine_cart_room::Y_INTERSECTION_MINE_CART_ROOM_DESCRIPTION,
};


pub(crate) mod back_door;
pub(crate) mod base_of_the_tree;
pub(crate) mod by_the_well;
pub(crate) mod cavern;
pub(crate) mod cavern_with_bucket;
pub(crate) mod claw_game;
pub(crate) mod claw_game_room;
pub(crate) mod connect_the_dots_wall;
pub(crate) mod credits;
pub(crate) mod dark_mine_cart_room;
pub(crate) mod darkness_closet;
pub(crate) mod darkness_door;
pub(crate) mod darkness_room;
pub(crate) mod door_room;
pub(crate) mod editor;
pub(crate) mod forest_path;
pub(crate) mod garden;
pub(crate) mod geyser;
pub(crate) mod gold_junction;
pub(crate) mod gold_vein;
pub(crate) mod grandfather_clock_gears;
pub(crate) mod grandfather_clock_room;
pub(crate) mod guardian_trees;
pub(crate) mod he_logo;
pub(crate) mod ig_logo;
pub(crate) mod inner_cavern;
pub(crate) mod interem_return_room;
pub(crate) mod inventory;
pub(crate) mod junction;
pub(crate) mod kitchen;
pub(crate) mod lab;
pub(crate) mod lab_exterior;
pub(crate) mod laundry;
pub(crate) mod living_room;
pub(crate) mod main_tree_house_hall;
pub(crate) mod middle_of_the_stream;
pub(crate) mod mine_cart_jumping_room;
pub(crate) mod mine_cart_lava_room;
pub(crate) mod mine_cart_room_1;
pub(crate) mod mine_cart_room_2;
pub(crate) mod mine_cart_switch_room;
pub(crate) mod mine_cart_winch;
pub(crate) mod mines_entrance;
pub(crate) mod music_room;
pub(crate) mod other_side_of_the_stream_entrance;
pub(crate) mod potion_table;
pub(crate) mod room_full_of_doors;
pub(crate) mod room_full_of_doors_2;
pub(crate) mod rooms;
pub(crate) mod saveload;
pub(crate) mod scheme;
pub(crate) mod shack_interior;
pub(crate) mod snare_path;
pub(crate) mod stair_room;
pub(crate) mod study;
pub(crate) mod swinging_room;
pub(crate) mod the_bridge;
pub(crate) mod the_dock;
pub(crate) mod the_entrance_to_the_dark_world;
pub(crate) mod the_rapids;
pub(crate) mod the_shack;
pub(crate) mod the_stone_bridge_entrance;
pub(crate) mod thermal_vents;
pub(crate) mod tic_tac_toe_room;
pub(crate) mod tik_tac_toe;
pub(crate) mod wild_gold_room;
pub(crate) mod y_intersection_mine_cart_room;
pub(crate) mod samsroom;

pub const ROOMS: &'static [RoomDescription] = &[
    HE_LOGO_DESCRIPTION,
    SAMS_ROOM_DESCRIPTION,
    THE_ENTRANCE_TO_THE_DARK_WORLD_DESCRIPTION,
    THE_BRIDGE_DESCRIPTION,
    SNARE_PATH_DESCRIPTION,
    JUNCTION_DESCRIPTION,
    THE_DOCK_DESCRIPTION,
    MIDDLE_OF_THE_STREAM_DESCRIPTION,
    OTHER_SIDE_OF_THE_STREAM_ENTRANCE_DESCRIPTION,
    FOREST_PATH_DESCRIPTION,
    GUARDIAN_TREES_DESCRIPTION,
    GARDEN_DESCRIPTION,
    THE_STONE_BRIDGE_ENTRANCE_DESCRIPTION,
    TIC_TAC_TOE_ROOM_DESCRIPTION,
    THE_SHACK_DESCRIPTION,
    SHACK_INTERIOR_DESCRIPTION,
    THE_RAPIDS_DESCRIPTION,
    CAVERN_DESCRIPTION,
    INNER_CAVERN_DESCRIPTION,
    BY_THE_WELL_DESCRIPTION,
    CAVERN_WITH_BUCKET_DESCRIPTION,
    GEYSER_DESCRIPTION,
    THERMAL_VENTS_DESCRIPTION,
    MINES_ENTRANCE_DESCRIPTION,
    MINE_CART_ROOM_1_DESCRIPTION,
    MINE_CART_ROOM_2_DESCRIPTION,
    Y_INTERSECTION_MINE_CART_ROOM_DESCRIPTION,
    CONNECT_THE_DOTS_WALL_DESCRIPTION,
    DARK_MINE_CART_ROOM_DESCRIPTION,
    MINE_CART_JUMPING_ROOM_DESCRIPTION,
    MINE_CART_WINCH_DESCRIPTION,
    MINE_CART_SWITCH_ROOM_DESCRIPTION,
    MINE_CART_LAVA_ROOM_DESCRIPTION,
    BACK_DOOR_DESCRIPTION,
    GOLD_JUNCTION_DESCRIPTION,
    WILD_GOLD_ROOM_DESCRIPTION,
    GOLD_VEIN_DESCRIPTION,
    CLAW_GAME_ROOM_DESCRIPTION,
    CLAW_GAME_DESCRIPTION,
    INTEREM_RETURN_ROOM_DESCRIPTION,
    BASE_OF_THE_TREE_DESCRIPTION,
    MAIN_TREE_HOUSE_HALL_DESCRIPTION,
    KITCHEN_DESCRIPTION,
    LIVING_ROOM_DESCRIPTION,
    DOOR_ROOM_DESCRIPTION,
    DARKNESS_DOOR_DESCRIPTION,
    ROOM_FULL_OF_DOORS_DESCRIPTION,
    ROOM_FULL_OF_DOORS_2_DESCRIPTION,
    GRANDFATHER_CLOCK_ROOM_DESCRIPTION,
    MUSIC_ROOM_DESCRIPTION,
    SWINGING_ROOM_DESCRIPTION,
    STUDY_DESCRIPTION,
    STAIR_ROOM_DESCRIPTION,
    LAB_EXTERIOR_DESCRIPTION,
    LAB_DESCRIPTION,
    POTION_TABLE_DESCRIPTION,
    DARKNESS_ROOM_DESCRIPTION,
    DARKNESS_CLOSET_DESCRIPTION,
    ROOMS_DESCRIPTION,
    INVENTORY_DESCRIPTION,
    SCHEME_DESCRIPTION,
    EDITOR_DESCRIPTION,
    GRANDFATHER_CLOCK_GEARS_DESCRIPTION,
    TIK_TAC_TOE_DESCRIPTION,
    LAUNDRY_DESCRIPTION,
    CREDITS_DESCRIPTION,
    IG_LOGO_DESCRIPTION,
    SAVELOAD_DESCRIPTION
];

use futures_util::future::Either;

use crate::game::engine::ScummEngine;

#[derive(Debug)]
pub struct ObjectDescription {
    pub id: i32,
    pub name: &'static str,
    on_clicked: Either<fn() -> Result<String, String>, &'static str>,
    pub click_offset: Option<(i32, i32)>,
}

impl ObjectDescription {
    const fn new_with_offset(
        id: i32,
        name: &'static str,
        on_clicked: fn() -> Result<String, String>,
        click_offset: Option<(i32, i32)>,
    ) -> Self {
        Self {
            id,
            name,
            on_clicked: Either::Left(on_clicked),
            click_offset,
        }
    }

    const fn new(id: i32, name: &'static str, on_clicked: fn() -> Result<String, String>) -> Self {
        Self {
            id,
            name,
            on_clicked: Either::Left(on_clicked),
            click_offset: None,
        }
    }

    const fn static_response(id: i32, name: &'static str, response: &'static str) -> Self {
        Self {
            id,
            name,
            on_clicked: Either::Right(response),
            click_offset: None,
        }
    }

    const fn placeholder(id: i32, name: &'static str) -> Self {
        Self {
            id,
            name,
            on_clicked: Either::Right("Not implemented yet. Try another object"),
            click_offset: None,
        }
    }

    pub fn click(&self, offset: Option<(i32, i32)>) -> Result<String, String> {
        let engine = unsafe { ScummEngine::get().ok_or("Engine not initialized".to_string()) }?;
        let objectData = engine
            .get_room_object(self.id as usize)
            .ok_or("Invalid Object".to_string())?;

        
        match &self.on_clicked {
            Either::Left(func) => func(),
            Either::Right(response) => Ok(response.to_string()),
        }
    }
}

#[derive(Debug)]
pub struct RoomDescription {
    pub id: i32,
    pub name: &'static str,
    pub on_entered: fn() -> String,
    pub objects: &'static [ObjectDescription],
    pub silent: bool,
}

impl RoomDescription {
    pub fn get_object(&self, id: i32) -> Option<&'static ObjectDescription> {
        self.objects.iter().find(|item| item.id == id)
    }

    pub const fn basic(id: i32, name: &'static str, objects: &'static [ObjectDescription]) -> Self {
        Self {
            id,
            name,
            on_entered: || "".to_string(),
            objects,
            silent: false
        }
    }

    pub const fn placeholder(id: i32, name: &'static str) -> Self {
        Self {
            id,
            name,
            on_entered: || "".to_string(),
            objects: &[],
            silent: false
        }
    }

    pub const fn silent(id: i32) -> Self {
        Self {
            id,
            name: "",
            on_entered: || "".to_string(),
            objects: &[],
            silent: true
        }
    }
}

pub const ROOMS: &'static [RoomDescription] = &[];

pub fn get_room_at(id: i32) -> Option<&'static RoomDescription> {
    ROOMS.iter().find(|item| item.id == id)
}

enum GetRoomContentsErr {
    EngineNotYetLoaded,
    RoomNotFound,
}

fn get_room_contents() -> Result<String, GetRoomContentsErr> {
    let mut readout = "".to_string();
    let engine = unsafe { ScummEngine::get() }.ok_or(GetRoomContentsErr::EngineNotYetLoaded)?;

    for object in get_room_at(engine.get_current_room_id())
        .ok_or(GetRoomContentsErr::RoomNotFound)?
        .objects
    {
        readout += &format!("- {} which as an ID of {}\n", object.name, object.id);
    }

    Ok(readout)
}

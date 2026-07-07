//! # Path: src/nodes/conveyor.rs

use bevy::prelude::*;
use crate::commons::*;
use crate::movables::item::Item;

#[derive(Component)]
pub struct Conveyor {
    pub item: Item
}
impl Registerable for Conveyor {
    fn register(_app: &mut App) {
        
    }
}

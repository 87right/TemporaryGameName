//! # Path: src/movables/item.rs

use bevy::prelude::*;
use crate::commons::*;

#[derive(Component)]
pub struct Item {
    pub id  : u64,
    pub size: u64,
}
impl Registerable for Item {
    fn register(_app: &mut App) {
    }
}

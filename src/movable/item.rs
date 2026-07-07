//! # Path: src/movable/item.rs

use bevy::prelude::*;

#[derive(Component)]
pub struct Item {
    pub id  : u64,
    pub size: u64,
}


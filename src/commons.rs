//! # Path: src/commons.rs

use bevy::prelude::*;

pub trait Registerable {
    fn register(app: &mut App);
}

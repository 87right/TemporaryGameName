//! # Path: src/movables/plugins.rs

use bevy::prelude::*;
use crate::commons::*;

pub struct MovablePlugins;
impl Plugin for MovablePlugins {
    fn build(&self, app: &mut App) {
        register::<crate::movables::item::Item>(app);
    }
}

fn register<T: Registerable>(app: &mut App) {
    T::register(app);
}

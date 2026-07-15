//! # Path: src/main.rs

#![allow(dead_code)]

mod grid;
mod constants;
mod commons;
mod camera;
mod nodes;
mod movables;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(crate::grid::plugins::GridPlugins)
        .add_plugins(crate::camera::plugins::CameraPlugins)
        .add_plugins(crate::nodes::plugins::NodePlugins)
        .add_plugins(crate::movables::plugins::MovablePlugins)
        .run();
}

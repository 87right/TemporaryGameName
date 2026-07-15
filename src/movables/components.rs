//! # Path: src/movables/components.rs

use std::collections::VecDeque;
use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity (Vec2);

#[derive(Component)]
pub struct MoveTasks{
    pub tasks: VecDeque<Movement>,
    pub timer: Timer
}

#[derive(Component)]
pub enum Movement{
    Bezier{begin: (Vec2, Vec2), end: (Vec2, Vec2), seconds: f32},
}

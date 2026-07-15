//! # Path: src/movables/plugins.rs

use bevy::prelude::*;
use crate::{commons::*, movables::components::{MoveTasks, Movement}};

pub struct MovablePlugins;
impl Plugin for MovablePlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, movement);

        register::<crate::movables::item::Item>(app);
    }
}

fn register<T: Registerable>(app: &mut App) {
    T::register(app);
}

fn movement(
    mut q: Query<(&mut MoveTasks, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut tasks, mut transform) in &mut q {
        if tasks.timer.tick(time.delta()).just_finished() {
            if let Some(task) = tasks.tasks.pop_front() {
                match task {
                    Movement::Bezier { begin: _, end, seconds: _ } => {
                        transform.translation = end.0.extend(3.);
                    }
                }
                if let Some(task) = tasks.tasks.get(0) {
                    match task {
                        Movement::Bezier { begin:_, end:_, seconds } => {
                            tasks.timer = Timer::from_seconds(*seconds, TimerMode::Once);
                        }
                    }
                }
            }
        } else {
            if let Some(task) = tasks.tasks.get(0) {
                match task {
                    Movement::Bezier { begin, end, seconds } => {
                        transform.translation = hermite_interpolate(
                            begin.0, 
                            begin.1, 
                            end.0, 
                            end.1, 
                            1. - tasks.timer.remaining().as_secs_f32() / seconds
                        ).extend(3.);
                    }
                }
            }
        }
    }
}

fn hermite_interpolate(p0: Vec2, v0: Vec2, p1: Vec2, v1: Vec2, t: f32) -> Vec2 {
    let t2 = t * t;
    let t3 = t2 * t;

    let h00 = 2.0 * t3 - 3.0 * t2 + 1.0;
    let h10 = t3 - 2.0 * t2 + t;
    let h01 = -2.0 * t3 + 3.0 * t2;
    let h11 = t3 - t2;

    h00 * p0 + h10 * v0 + h01 * p1 + h11 * v1
}

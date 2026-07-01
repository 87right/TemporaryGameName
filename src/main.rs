use bevy::prelude::*;
use std::collections::HashMap;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Setup)
        .run();
}

#[derive(Resource, Default)]
struct WorldGrid (HashMap<IVec2, Entity>);

#[derive(Component)]
enum GridState {
    Empty,
}

#[derive(Component)]
struct GridPos (IVec2);

pub struct Setup;
impl Plugin for Setup {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldGrid::default());
        app.add_systems(Startup, world_gen);
    }
}

fn world_gen(mut world_grid: ResMut<WorldGrid>, mut commands: Commands) {
    for y in 0..100 {
        for x in 0..100{
            let v = IVec2 {x, y};
            let id = commands.spawn((GridPos (v), GridState::Empty)).id();
            world_grid.0.insert(v, id);
        }
    }
}

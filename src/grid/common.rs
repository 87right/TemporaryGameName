use bevy::prelude::*;
use crate::grid::resource::GridEntityMap;

pub trait BasicNode {
    fn remove(commands: &mut EntityCommands);
    fn spawn(grid_entity_map: Res<GridEntityMap>, commands: &mut Commands);
    fn get_id() -> String;
}

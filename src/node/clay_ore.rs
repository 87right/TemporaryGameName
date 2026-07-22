use bevy::prelude::*;

use crate::grid::{common::BasicNode, component::{LeftClicked, TextureBuff}, system_set::GridFixed};

#[derive(Component)]
pub struct ClayOre;
impl BasicNode for ClayOre {
    fn get_id() -> String {
        "clay_ore".to_string()
    }
    fn remove(commands: &mut bevy::ecs::system::EntityCommands) {
        commands.remove::<ClayOre>();
    }
    fn spawn(commands: &mut bevy::ecs::system::Commands, entity: bevy::ecs::entity::Entity) {
        commands.entity(entity).insert((
            ClayOre,
            TextureBuff("textures/tile/clay_ore.png".to_string()),
        ));
    }
    fn register(app: &mut bevy::app::App) {
        app.add_systems(FixedUpdate, on_left_clicked.in_set(GridFixed::MainUpdate));

    }
}


fn on_left_clicked(
    node_q: Query<Entity, (With<LeftClicked>, With<ClayOre>)>,
) {
    for entity in node_q {
    }
}

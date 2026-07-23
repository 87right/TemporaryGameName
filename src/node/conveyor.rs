use bevy::prelude::*;

use crate::{
    grid::{common::*, component::*, resource::*, system_set::*, util::*},
    node::*,
};

#[derive(Component)]
pub struct Conveyor {
    from: Direction,
    to: Direction,
}
impl BasicNode for Conveyor {
    fn get_id() -> String {
        "conveyor".to_string()
    }
    fn register(app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                on_placed.in_set(GridFixed::OnPlaced),
                on_left_clicked.in_set(GridFixed::MainUpdate),
            ),
        );
    }
    fn remove(commands: &mut EntityCommands) {
        commands.remove::<Conveyor>();
    }
    fn spawn(commands: &mut Commands, entity: Entity) {
        commands.entity(entity).insert((
            Conveyor {
                from: Direction::NegX,
                to: Direction::NegX,
            },
            TextureBuff("textures/tile/conveyor_0_0.png".to_string()),
        ));
    }
}

fn on_placed(
    mut commands: Commands,
    mut self_q: Query<(&mut Conveyor, &GridPos)>,
    placed_q: Query<Entity, With<Placed>>,
    grid: Res<GridEntityMap>,
) {
    for e in placed_q {
        let mut new_from = Direction::NegX;
        let mut new_to = Direction::NegX;
        if let Ok((_, pos)) = self_q.get(e) {
            for dir in Direction::ALL {
                if let Some(cur_c) = grid.get(&(*pos + dir.into_grid_pos()))
                    && let Ok((cur_c, _)) = self_q.get(cur_c)
                {
                    if cur_c.from == dir.inverse() {
                        new_to = dir;
                    } else if cur_c.to == dir.inverse() {
                        new_from = dir;
                    }
                }
            }
        }
        if let Ok((mut c, _)) = self_q.get_mut(e) {
            c.from = new_from;
            c.to = new_to;
            commands.entity(e).insert(TextureBuff(
                format!(
                    "textures/tile/conveyor_{}_{}.png",
                    c.from.get_id(),
                    c.to.get_id()
                )
                .to_string(),
            ));
        }
    }
}

fn on_left_clicked(
    mut commands: Commands,
    conveyor_q: Query<(&mut Conveyor, Entity), With<LeftClicked>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (mut c, e) in conveyor_q {
        if keys.pressed(KeyCode::ControlLeft) {
            replace::<air::Air>(&mut commands, e);
        }

        let mut new_dir = Direction::NegX;
        if keys.pressed(KeyCode::KeyS) {
            new_dir = Direction::NegY;
        } else if keys.pressed(KeyCode::KeyD) {
            new_dir = Direction::X;
        } else if keys.pressed(KeyCode::KeyW) {
            new_dir = Direction::Y;
        } else if !keys.pressed(KeyCode::KeyA) {
            continue;
        }
        if keys.pressed(KeyCode::ShiftLeft) {
            c.to = new_dir;
        } else {
            c.from = new_dir;
        }
        commands.entity(e).insert(TextureBuff(
            format!(
                "textures/tile/conveyor_{}_{}.png",
                c.from.get_id(),
                c.to.get_id()
            )
            .to_string(),
        ));
    }
}

use bevy::{
    prelude::*,
};
use crate::grid::{
        component::{GridPos, PlaceBuff}, message::{
            LeftClicked, Placed, RightClicked
        }, resource::{
            GridEntityMap, 
            SpawnTable
        }, system_set::*,
    };

pub struct GridPlugin;
impl Plugin for GridPlugin{
    fn build(&self, app: &mut App) {
        register_grid_update_schedule(app);
    }
}

fn register_grid_update_schedule(app: &mut App) {
    app.insert_resource(Time::<Fixed>::from_hz(1.0));
    app.configure_sets(FixedUpdate, (
        GridFixed::OnPlaced,
        GridFixed::IOReserve.after(GridFixed::OnPlaced),
        GridFixed::IOExecute.after(GridFixed::IOReserve),
        GridFixed::MainUpdate.after(GridFixed::IOExecute),
        GridFixed::Cleanup.after(GridFixed::MainUpdate),
        GridFixed::OnRemoved.after(GridFixed::Cleanup),
    ));
}

fn consume_place_buff(
    mut commands: Commands,
    mut placed_message_writer: MessageWriter<Placed>,
    place_buff_q: Query<(&PlaceBuff, Entity)>,
    spawn_table: Res<SpawnTable>,
) {
    for (buff, e) in place_buff_q {
        if let Some(spawn_fn) = spawn_table.get(&buff.0) {
            spawn_fn(&mut commands, e);
            placed_message_writer.write(Placed(e));
        }
    }
}

fn handle_mouse_click(
    mut right_clicked_writer: MessageWriter<RightClicked>,
    mut left_clicked_writer: MessageWriter<LeftClicked>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    grid_entity_map: Res<GridEntityMap>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
) {
    let lc = mouse_buttons.just_released(MouseButton::Left);
    let rc = mouse_buttons.just_released(MouseButton::Right);
    if lc || rc {
        let (camera, global_transform) = camera.into_inner();
        if let Some(cursor_pos) = window.cursor_position() {
            if let Ok(cursor_pos) = camera.viewport_to_world_2d(global_transform, cursor_pos)
            && let grid_pos = GridPos::from_world_pos(cursor_pos)
            && let Some(entity) = grid_entity_map.get(&grid_pos) {
                if lc {
                    left_clicked_writer.write(LeftClicked(entity));
                }
                if rc {
                    right_clicked_writer.write(RightClicked(entity));
                }
            }
        }
    }
}

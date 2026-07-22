use bevy::{
    prelude::*,
};
use crate::grid::{
        component::{
            *
        }, resource::{
            Background, GridEntityMap, GridGenSetting, SpawnTable, SyncMouseButtonInput
        }, system_set::*, util::{reload_background, respawn_grid},
    };

pub struct GridPlugin;
impl Plugin for GridPlugin{
    fn build(&self, app: &mut App) {
        register_grid_update_schedule(app);
        add_resource(app);
        app.add_systems(Startup, (
            respawn_grid,
            reload_background,
        ));
        app.add_systems(Update, (
            recieve_update_input,
            handle_mouse_click,
        ));
        app.add_systems(FixedUpdate, consume_place_buff);
        app.add_systems(FixedLast, (
            clear_mid_fixed_input,
            clear_message_component,
        ));
    }
}

fn register_grid_update_schedule(app: &mut App) {
    app.insert_resource(Time::<Fixed>::from_hz(20.0));
    app.configure_sets(FixedUpdate, (
        GridFixed::OnPlaced,
        GridFixed::IOReserve.after(GridFixed::OnPlaced),
        GridFixed::IOExecute.after(GridFixed::IOReserve),
        GridFixed::MainUpdate.after(GridFixed::IOExecute),
        GridFixed::OnRemoved.after(GridFixed::MainUpdate),
        GridFixed::Cleanup.after(GridFixed::OnRemoved),
    ));
}

fn clear_message_component(
    mut commands: Commands,
    lc: Query<Entity, With<LeftClicked>>,
    rc: Query<Entity, With<RightClicked>>,
    pl: Query<Entity, With<Placed>>,
    rm: Query<Entity, With<Removed>>,
) {
    for e in lc {
        commands.entity(e).remove::<LeftClicked>();
    }
    for e in rc {
        commands.entity(e).remove::<RightClicked>();
    }
    for e in pl {
        commands.entity(e).remove::<Placed>();
    }
    for e in rm {
        commands.entity(e).remove::<Removed>();
    }
}

fn add_resource(app: &mut App) {
    app.insert_resource(GridEntityMap::default());
    app.insert_resource(SpawnTable::default());
    app.insert_resource(GridGenSetting::default());
    app.insert_resource(Background::default());
    app.insert_resource(SyncMouseButtonInput::default());
}

fn consume_place_buff(
    mut commands: Commands,
    place_buff_q: Query<(&PlaceBuff, Entity)>,
    spawn_table: Res<SpawnTable>,
) {
    for (buff, e) in place_buff_q {
        if let Some(spawn_fn) = spawn_table.get(&buff.0) {
            spawn_fn(&mut commands, e);
            commands.entity(e).insert(Placed);
        }
    }
}

fn recieve_update_input(
    mut sync_mouse_button: ResMut<SyncMouseButtonInput>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    sync_mouse_button.write(&mouse_button);
}

fn clear_mid_fixed_input(
    mut sync_mouse_button: ResMut<SyncMouseButtonInput>,
) {
    sync_mouse_button.clear();
}

fn handle_mouse_click(
    mut commands: Commands,
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
                    commands.entity(entity).insert(LeftClicked);
                }
                if rc {
                    commands.entity(entity).insert(RightClicked);
                }
            }
        }
    }
}

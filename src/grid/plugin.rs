use bevy::{
    prelude::*,
};
use crate::grid::{
        component::PlaceBuff,
        message::Placed, 
        resource::SpawnTable, 
        system_set::*,
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

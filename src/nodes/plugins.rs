//! # Path: src/nodes/plugins.rs

use bevy::prelude::*;
use crate::constants::PIE;
use crate::grid::components::GridPos;
use crate::grid::components::WorldGrid;
use crate::nodes::commons::InputPort;
use crate::nodes::commons::Inventory;
use crate::nodes::commons::ItemSendReq;
use crate::nodes::commons::OutputPort;
use crate::nodes::commons::Shake;
use crate::nodes::*;
use crate::commons::*;

pub struct NodePlugins;
impl Plugin for NodePlugins {
    fn build(&self, app: &mut App) {
        app.add_message::<ItemSendReq>();

        app.add_systems(First, handle_auto_input);
        app.add_systems(Last, handle_auto_output);
        app.add_systems(Update, shaking);

        register::<empty::Empty>(app);
        register::<clay_ore::ClayOre>(app);
        register::<conveyor::Conveyor>(app);
        register::<item_collector::ItemCollector>(app);
        register::<clay_furnace::ClayFurnace>(app);
    }
}

fn register<T: Registerable> (app: &mut App) {
    T::register(app);
}

fn shaking(
    mut command: Commands,
    q: Query<(&mut Shake, &mut Transform, Entity)>,
    time: Res<Time>,
) {
    for (mut shake, mut transform, entity) in q {
        if shake.timer.tick(time.delta()).just_finished() {
            command.entity(entity).remove::<Shake>();
        }
        transform.translation.x = shake.base_x + shake.scale * (shake.timer.remaining().as_secs_f32() * 2. * PIE / shake.pace).sin();
    }
}

fn handle_auto_input(
    mut reader: MessageReader<ItemSendReq>,
    mut input_q: Query<&mut InputPort>,
    mut inv_q: Query<&mut Inventory>,
) {
    for m in reader.read() {
        if let Ok(mut port) = input_q.get_mut(m.to) {
            let mut can_process = false;
            if let Ok(from_inv) = inv_q.get(m.from)
            && let Ok(to_inv) = inv_q.get(m.to) 
            && let Some(try_item) = from_inv.check_item(m.index) {
                can_process = to_inv.can_insert(*port, *try_item);
            }
            if !can_process {continue;}
            let item = if let Ok(mut inv) = inv_q.get_mut(m.from) {
                inv.take_item(m.index)
            } else {
                None
            };
            if let Some(item) =  item
            && let Ok(mut to_inv) = inv_q.get_mut(m.to){
                to_inv.insert(item);
                port.recieved = true;
                port.display_item = m.e_item;
            }
        }
    }
}

fn handle_auto_output(
    mut writer: MessageWriter<ItemSendReq>,
    output_q: Query<(&mut OutputPort, &Inventory, &GridPos, Entity)>,
    world_grid: Res<WorldGrid>,
) {
    for (mut port, inventory, grid_pos, e) in output_q {
        if let Some(slot_id) = inventory.some_item(port.port) 
        && let Some(to) = world_grid.0.get(&(port.to + grid_pos.0)) {
            writer.write(ItemSendReq { 
                from: e, 
                to: *to, 
                index: slot_id,
                e_item: port.display_item
            });
            port.sent = true;
        } 
    }
}

//! # Path: src/nodes/conveyor.rs

use bevy::prelude::*;
use crate::commons::*;
use crate::grid::components::{GridPos, WorldGrid};
use crate::grid::messages::*;
use crate::nodes::commons::*;

#[derive(Component)]
pub struct Conveyor;
impl Registerable for Conveyor {
    fn register(app: &mut App) {
        app.add_systems(Update, (
            on_item_sent,
            on_update,
        ));
        app.add_systems(PostUpdate, on_placed);
    }
}
impl Spawnable for Conveyor {
    fn get_bundle() -> impl Bundle {
        (
            Conveyor,
            Inventory::create_empty(InventorySize(1))
        )
    }
}

fn on_item_sent(
    mut reader: MessageReader<ItemSendReq>,
    mut inventory_q: Query<&mut Inventory>,
    conveyor_q: Query<&GridPos, With<Conveyor>>,
    world: Res<WorldGrid>,
) {
    for m in reader.read() {
        let can_receive = if let Ok(inventory) = inventory_q.get(m.to) 
        && let Ok(grid_pos) = conveyor_q.get(m.to) 
        && let Some(target_e) = world.0.get(&(grid_pos.0 + IVec2::X)){
            *target_e == m.from && inventory.check_item(InventorySlotID(0)).is_none()
        } else {
            false
        };

        if !can_receive {continue;}

        if let Ok([mut to_inv, mut from_inv]) = inventory_q.get_many_mut([m.to, m.from]) {
            to_inv.write_item(InventorySlotID(0), InventorySlot(from_inv.take_item(&m.index)));
        }
    }
}

fn on_update(
    mut writer: MessageWriter<ItemSendReq>,
    conveyor_q: Query<(&mut Inventory, &GridPos, Entity), With<Conveyor>>,
    world: Res<WorldGrid>
) {
    for (inventory, grid_pos, e) in conveyor_q {
        if inventory.check_item(InventorySlotID(0)).is_some() 
        && let Some(to) = world.0.get(&(grid_pos.0 + IVec2::NEG_X)) {
            writer.write(ItemSendReq {
                from: e,
                to: *to,
                index: InventorySlotID(0),
            });
        }
    }
}

fn on_placed(
    mut reader: MessageReader<Placed>,
    mut q : Query<&mut Sprite, With<Conveyor>>,
    asset_server: Res<AssetServer>,
) {
    for m in reader.read() {
        let clicked_entity = m.0;
        if let Ok(mut sprite) = q.get_mut(clicked_entity) {
            *sprite = Sprite::from_image(
                asset_server.load("textures/tile/conveyor.png")
            );
        }
    }
}

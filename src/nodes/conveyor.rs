//! # Path: src/nodes/conveyor.rs

use bevy::prelude::*;
use crate::commons::*;
use crate::grid::messages::*;
use crate::nodes::commons::*;

#[derive(Component)]
pub struct Conveyor;
impl Registerable for Conveyor {
    fn register(app: &mut App) {
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
    mut conveyor_q: Query<&mut Inventory, With<Conveyor>>,
    mut inventory_q: Query<&mut Inventory>,
) {
    for m in reader.read() {
        if let Ok(mut inventory) = conveyor_q.get_mut(m.to) {
            if let Some(_) = inventory.check_item(InventorySlotID(0)) {
                return;
            }
            if let Ok(mut from_inventory) = inventory_q.get_mut(m.from) {
                inventory.write_item(InventorySlotID(0), InventorySlot(from_inventory.take_item(&m.index)));
            }
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

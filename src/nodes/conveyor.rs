//! # Path: src/nodes/conveyor.rs

use bevy::prelude::*;
use crate::commons::*;
use crate::constants::CELL_SIZE;
use crate::grid::components::GridPos;
use crate::grid::messages::*;
use crate::movables::components::{MoveTasks, Movement};
use crate::movables::item::DisplayItem;
use crate::nodes::commons::*;

#[derive(Component)]
pub struct Conveyor {
    timer: Timer,
    display_item: Option<Entity>,
}
impl Registerable for Conveyor {
    fn register(app: &mut App) {
        app.add_systems(Update, (
            on_input,
            on_output,
            on_update,
        ));
        app.add_systems(PostUpdate, on_placed);
    }
}
impl Spawnable for Conveyor {
    fn get_bundle() -> impl Bundle {
        (
            Conveyor {
                timer: Timer::from_seconds(2., TimerMode::Once),
                display_item: None
            },
            Inventory::create_empty(InventorySize(2)),
            InputPort::new(
                Port::Single(InventorySlotID(0))
            ),
            OutputPort::new(
                Port::Single(InventorySlotID(1)), 
                IVec2::NEG_X
            )
        )
    }
}

fn on_update(
    conveyor_q: Query<(&mut Inventory, &mut Conveyor)>,
    time: Res<Time>,
) {
    for (mut inventory, mut conveyor) in conveyor_q {
        if inventory.check_item(InventorySlotID(0)).is_some() {
            println!("ちゃんと動いてるよ!");
            if conveyor.timer.tick(time.delta()).just_finished() {
                let item = inventory.check_item(InventorySlotID(0)).and_then(|x| Some(*x));
                inventory.write_item(
                    InventorySlotID(1),
                    InventorySlot(item)
                );
                println!("送ってるはずだよ!");
            }
        } 
    }
}

fn on_input(
    mut commands: Commands,
    conveyor_q: Query<(&mut InputPort, &mut OutputPort, &mut Conveyor, &GridPos, &Inventory)>,
    asset_server: Res<AssetServer>,
) {
    for (mut port, mut out, mut conveyor, grid_pos, inventory) in conveyor_q {
        if port.recieved {
            port.recieved = false;
            conveyor.timer.reset();

            let item_pos = grid_pos.to_center_vec2();

            if let Some(e_item) = port.display_item.take() {
                conveyor.display_item = Some(e_item);
            }else if let Some(item) = inventory.0.get(0).and_then(|x| x.0) {
                conveyor.display_item = Some(commands.spawn((
                    DisplayItem,
                    item.id.get_sprite(&asset_server),
                    Transform::from_xyz(
                        item_pos.x + CELL_SIZE as f32 / 2.,
                        item_pos.y,
                        1.
                    ),
                )).id());
            }

            out.display_item = conveyor.display_item;

            if let Some(item) = conveyor.display_item {
                commands.entity(item).insert(
                MoveTasks{
                        tasks: vec![
                            Movement::Bezier { 
                                begin: (
                                    item_pos + Vec2{x: CELL_SIZE / 2., y: 0.},
                                    Vec2{x: -CELL_SIZE, y: 0.},
                                ), 
                                end: (
                                    item_pos + Vec2{x: - CELL_SIZE / 2., y: 0.},
                                    Vec2{x: -CELL_SIZE, y: 0.},
                                ), 
                                seconds: 2.
                            }].into(),
                        timer: Timer::from_seconds(2., TimerMode::Once)
                    },
                );
            }
        }
    }
}

fn on_output(
    conveyor_q: Query<(&mut OutputPort, &mut Inventory, &mut Conveyor)>
) {
    for (mut port, mut inventory, mut conveyor) in conveyor_q {
        if port.sent {
            port.sent = false;
            inventory.take_item(InventorySlotID(0));
            conveyor.display_item.take();
            port.display_item.take();
        }
    }
}

fn on_placed(
    mut commands: Commands,
    mut reader: MessageReader<Placed>,
    mut q : Query<(), With<Conveyor>>,
    asset_server: Res<AssetServer>,
) {
    for m in reader.read() {
        let clicked_entity = m.0;
        if q.get_mut(clicked_entity).is_ok() {
            commands.entity(clicked_entity).insert(Sprite::from_image(
                asset_server.load("textures/tile/conveyor.png")
            ));
        }
    }
}

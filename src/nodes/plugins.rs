//! # Path: src/nodes/plugins.rs

use bevy::prelude::*;
use crate::nodes::commons::ItemSendReq;
use crate::nodes::*;
use crate::commons::*;

pub struct NodePlugins;
impl Plugin for NodePlugins {
    fn build(&self, app: &mut App) {
        app.add_message::<ItemSendReq>();

        register::<empty::Empty>(app);
        register::<clay_ore::ClayOre>(app);
        register::<conveyor::Conveyor>(app);
        register::<item_collector::ItemCollector>(app);
    }
}

fn register<T: Registerable> (app: &mut App) {
    T::register(app);
}

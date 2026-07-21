use bevy::prelude::*;

use crate::grid::{
    common::BasicNode, 
    component::PlaceBuff, 
    message::Removed
};

pub fn replace<To: BasicNode>(
    commands: &mut Commands,
    removed_message_writer: &mut MessageWriter<Removed>,
    e: Entity, 
) {
    removed_message_writer.write(Removed(e));
    commands.entity(e).insert(PlaceBuff(To::get_id()));
}

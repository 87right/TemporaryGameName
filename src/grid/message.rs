use bevy::ecs::{entity::Entity, message::Message};

#[derive(Message)]
pub struct Removed(pub Entity);

#[derive(Message)]
pub struct Placed(pub Entity);

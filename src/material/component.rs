use bevy::prelude::*;

#[derive(Component)]
pub struct Channel<T: Component> {
    pub input: Vec<Port<T>>,
    pub output: Vec<Port<T>>,
    pub gather: Vec<Port<T>>,
}

#[derive(Component)]
pub struct Port<T: Component> {
    pub filter: Option<Filter<T>>,
}

#[derive(Component)]
pub enum Filter<T: Component> {
    Specific { val: T },
}

#[derive(Component)]
pub struct Inventory<T: Component>(pub Vec<MaterialSlot<T>>);

#[derive(Component)]
pub struct MaterialSlot<T: Component>(pub Option<T>);

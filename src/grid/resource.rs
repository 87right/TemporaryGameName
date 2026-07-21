use std::collections::HashMap;
use bevy::prelude::*;
use crate::{
    grid::{
        component::*
    }
};


#[derive(Resource)]
pub struct GridEntityMap(pub HashMap<IVec2, Entity>);
impl GridEntityMap{
    pub fn get(&self, grid_pos: &GridPos) -> Option<Entity> {
        self.0.get(&grid_pos.0).and_then(|e| Some(*e))
    }
    pub fn insert(&mut self, grid_pos: &GridPos, entity: Entity) -> Option<Entity> {
        self.0.insert(grid_pos.0.clone(), entity)
    }
}

#[derive(Resource)]
pub struct GridGenSetting{
    
}
impl Default for GridGenSetting{
    fn default() -> Self {
        Self{

        }
    }
}

#[derive(Resource)]
pub struct SpawnTable(pub HashMap<String, fn(&mut Commands, Entity)>);
impl SpawnTable {
    pub fn insert(&mut self, key: String, val: fn(&mut Commands, Entity)) -> Option<fn(&mut Commands, Entity)>{
        self.0.insert(key, val)
    }
    pub fn get(&self, key: &String) -> Option<&fn(&mut Commands, Entity)> {
        self.0.get(key)
    }
}

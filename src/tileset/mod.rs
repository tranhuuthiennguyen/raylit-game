use std::collections::HashMap;

use serde::Deserialize;

pub mod tile;

#[derive(Debug, Deserialize)]
pub struct LevelData {
    pub tilewidth: u32,
    pub tileheight: u32,
    pub width: u32,
    pub height: u32,
    pub tilesets: Vec<TileSetData>,
    pub layers: Vec<LayerData>
    
}
#[derive(Debug, Deserialize)]
pub struct TileSetData {
    pub name: String,
    pub firstgrid: u32,
    pub tilewidth: u32,
    pub tileheight: u32,
    pub imagewidth: u32,
    pub imageheight: u32,
    pub source: String,
    pub columns: u32,
    pub tilecount: u32,
    pub tiles: HashMap<usize, TileData>,
}

#[derive(Debug, Deserialize)]
pub struct TileData {
    pub label: String,
    pub solid: bool
}

#[derive(Debug, Deserialize)]
pub struct LayerData {
    pub name: String,
    pub z_order: u32,
    pub data: Vec<usize>
}

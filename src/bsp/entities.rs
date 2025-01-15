use glam::Vec3;

use super::{Bsp, BspHeader};

#[derive(Debug)]
pub struct Entity {
    pub classname: String,
    pub origin: Vec3,
    pub angle: u32,
    pub light: u32,
    pub target: String,
    pub killtarget: String,
    pub spawnflags: u32,
    pub style: u32,
    pub message: String,
    pub mangle: Vec3,
    pub speed: u32,
    pub wait: u32,
    pub lip: u32,
    pub damage: u32,
    pub health: u32,
    pub delay: u32,
    pub sounds: u32,
    pub wad: String,
    pub height: u32,
}

impl Bsp {
    pub fn read_entities(&self, header: &BspHeader) -> Vec<Entity> {
        // Assuming the BSP has an `entities` field that is a string containing all entities
        let 
        let mut entities = Vec::new();

        // Extract the entity data from the BSP file using the header
       



            entities.push(entity);
        }

        entities
    }
}

fn parse_key_value(line: &str) -> Option<(String, String)> {
    let mut parts = line.splitn(2, '\"');
    parts.next()?; // Skip the initial opening quote
    let key = parts.next()?.trim().to_string();
    parts.next()?; // Skip the quote between key and value
    let value = parts.next()?.trim().to_string();
    Some((key, value))
}

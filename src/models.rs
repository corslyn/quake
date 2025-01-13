use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt};
use glam::Vec3;

pub struct ModelHeader {
    pub ident: u32,   // "IDPO"
    pub version: u32, // 6
    pub scale: Vec3,
    pub scale_origin: Vec3,
    pub boundingradius: f32,
    pub eyeposition: Vec3,
    pub numskins: u32,
    pub skinwidth: u32,
    pub skinheight: u32,
    pub numverts: u32,
    pub numtriangles: u32,
    pub numframes: u32,
    pub synctype: u32,
    pub flags: u32,
    pub size: f32,
}

pub struct Model {
    pub header: ModelHeader,
    pub skin_data: Vec<u8>,
    pub skin_vertices: Vec<u8>,
    pub model_triangles: Vec<u8>,
    // pub model_frames
}

impl ModelHeader {
    pub fn from_reader<R: std::io::Read>(reader: &mut R) -> Result<Self, std::io::Error> {
        let ident = reader.read_u32::<LittleEndian>()?;
        let version = reader.read_u32::<LittleEndian>()?;

        if ident != u32::from_le_bytes(*b"IDPO") || version != 6 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid MDL file",
            ));
        }

        let scale = Vec3::new(
            reader.read_f32::<LittleEndian>()?,
            reader.read_f32::<LittleEndian>()?,
            reader.read_f32::<LittleEndian>()?,
        );
        let scale_origin = Vec3::new(
            reader.read_f32::<LittleEndian>()?,
            reader.read_f32::<LittleEndian>()?,
            reader.read_f32::<LittleEndian>()?,
        );
        let boundingradius = reader.read_f32::<LittleEndian>()?;
        let eyeposition = Vec3::new(
            reader.read_f32::<LittleEndian>()?,
            reader.read_f32::<LittleEndian>()?,
            reader.read_f32::<LittleEndian>()?,
        );

        let numskins = reader.read_u32::<LittleEndian>()?;
        let skinwidth = reader.read_u32::<LittleEndian>()?;
        let skinheight = reader.read_u32::<LittleEndian>()?;
        let numverts = reader.read_u32::<LittleEndian>()?;
        let numtriangles = reader.read_u32::<LittleEndian>()?;
        let numframes = reader.read_u32::<LittleEndian>()?;
        let synctype = reader.read_u32::<LittleEndian>()?;
        let flags = reader.read_u32::<LittleEndian>()?;
        let size = reader.read_f32::<LittleEndian>()?;

        Ok(Self {
            ident,
            version,
            scale,
            scale_origin,
            boundingradius,
            eyeposition,
            numskins,
            skinwidth,
            skinheight,
            numverts,
            numtriangles,
            numframes,
            synctype,
            flags,
            size,
        })
    }
}

pub fn parse_skins<R: std::io::Read>(
    reader: &mut R,
    header: &ModelHeader,
) -> Result<Vec<u8>, std::io::Error> {
    let skin_size = (header.skinwidth * header.skinheight) as usize;
    let mut skins = vec![0; skin_size];
    reader.read_exact(&mut skins)?;
    Ok(skins)
}

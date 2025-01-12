use std::{fs::File, io, io::Read};

use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug)]
pub struct Pak {
    /// Raw pak data
    pub data: Vec<u8>,
}

pub struct PakHeader {
    pub id: String,      // Should be "PACK"
    pub dir_offset: u32, // Offset to the directory
    pub dir_size: u32,   // Size of the directory
}

impl Pak {
    /// Creates a PAK from a file
    pub fn new(filepath: &str) -> io::Result<Self> {
        let mut pak_file = File::open(filepath)?;
        let mut data: Vec<u8> = Vec::new();
        pak_file.read_to_end(&mut data)?;
        Ok(Pak { data })
    }

    /// Returns a PAK Header
    pub fn read_header(&self) -> io::Result<PakHeader> {
        let mut cursor = io::Cursor::new(&self.data);

        // Read the first 4 bytes as a string
        let mut id_buf = [0u8; 4];
        cursor.read_exact(&mut id_buf)?;
        let id = String::from_utf8_lossy(&id_buf).to_string();

        if id != "PACK" {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unsupported PAK file",
            ));
        }

        // Read directory offset and size
        let dir_offset = cursor.read_u32::<LittleEndian>()?;
        let dir_size = cursor.read_u32::<LittleEndian>()?;

        Ok(PakHeader {
            id,
            dir_offset,
            dir_size,
        })
    }
}

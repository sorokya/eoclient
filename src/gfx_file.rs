use std::collections::HashMap;
use std::io::Cursor;

use pelite::FileMap;
use pelite::pe32::{Pe, PeFile};
use pelite::resources::Name;

use byteorder::{LittleEndian, WriteBytesExt, ReadBytesExt};

const BMP_FILE_HEADER_SIZE: usize = 14;

pub struct GfxFile {
    bitmaps: HashMap<u32, Vec<u8>>,
}

impl GfxFile {
    pub fn new(id: u32) -> Self {
        let file_map = FileMap::open(&format!("gfx/gfx{:03}.egf", id)).unwrap();
        let file = PeFile::from_bytes(file_map.as_ref()).unwrap();
        let resources = file.resources().unwrap();
        let root = resources.root().unwrap();
        let bitmaps_dir = root.get(Name::Id(2)).unwrap().dir().unwrap();
        let mut bitmaps = HashMap::new();

        for entry in bitmaps_dir.entries() {
            if let Name::Id(id) = entry.name().unwrap() {
                let data_entry = entry.entry().unwrap().dir().unwrap().first_data().unwrap();

                let image_bytes = data_entry.bytes().unwrap();
                let mut bitmap: Vec<u8> = Vec::with_capacity(image_bytes.len() + BMP_FILE_HEADER_SIZE);
                bitmap.extend_from_slice(b"BM");
                bitmap.write_u32::<LittleEndian>(image_bytes.len() as u32).unwrap();
                bitmap.write_u16::<LittleEndian>(0).unwrap();
                bitmap.write_u16::<LittleEndian>(0).unwrap();

                let info_header_bytes = image_bytes[0..2].to_vec();
                let mut info_header_cursor = Cursor::new(info_header_bytes);
                let info_header_size = info_header_cursor.read_u16::<LittleEndian>().unwrap() as usize;
                bitmap.write_u32::<LittleEndian>((BMP_FILE_HEADER_SIZE + info_header_size) as u32).unwrap();
                bitmap.extend_from_slice(&image_bytes[0..]);
                bitmaps.insert(id, bitmap);
            }
        }

        Self {
            bitmaps,
        }
    }

    pub fn get_bitmap_at_id(&self, id: u32) -> Option<&Vec<u8>> {
        self.bitmaps.get(&id)
    }
}
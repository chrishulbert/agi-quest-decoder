// This is for loading the directory files eg logdir, picdir, viewdir, and snddir.

use anyhow::Result;

pub struct Directories {
    pub logic: Directory,
    pub pictures: Directory,
    pub views: Directory,
    pub sounds: Directory,
}

pub struct Directory {
    pub entries: Vec<Option<Entry>>, // Option because 0xffffff are 'no data' slots.
}

#[derive(Debug)]
pub struct Entry {
    pub volume: usize, // Eg 1 means the contents are in the 'vol.1' file.
    pub position: usize,
}

impl Directories {
    pub fn read(path: &str) -> Result<Directories> {
        Ok(Directories {
            logic:    Directory::parse(&std::fs::read(format!("{}/logdir",  path))?),
            pictures: Directory::parse(&std::fs::read(format!("{}/picdir",  path))?),
            views:    Directory::parse(&std::fs::read(format!("{}/viewdir", path))?),
            sounds:   Directory::parse(&std::fs::read(format!("{}/snddir",  path))?),
        })
    }
}

impl Directory {
    fn parse(data: &[u8]) -> Directory {
        Directory {
            entries: data.chunks_exact(3).map(Entry::parse).collect(),
        }
    }
}

impl Entry {
    fn parse(data: &[u8]) -> Option<Entry> {
        if data[0] == 0xff && data[1] == 0xff && data[2] == 0xff {
            return None;
        }
        let volume = data[0] >> 4;
        let position: usize = 
            (((data[0] & 0xf) as usize) << 16) +
            ((data[1] as usize) << 8) +
            (data[2] as usize);
        Some(Entry { volume: volume as usize, position })
    }
}

// This is responsible for loading the volumes into memory.

use anyhow::Result;

pub struct Volumes {
    pub volumes: Vec<Vec<u8>>,
}

pub fn read(path: &str) -> Result<Volumes> {
    let mut volumes: Vec<Vec<u8>> = vec![];
    for i in 0..9999 {
        let vol_path = format!("{}/vol.{}", path, i);
        let exists = std::fs::exists(&vol_path)?;
        if !exists { break }
        let content = std::fs::read(&vol_path)?;
        volumes.push(content);
    }
    Ok(Volumes { volumes })
}

// This is responsible for using the directories to split up the volumes into individual resources.

use anyhow::{Result, bail};

use crate::volumes::Volumes;
use crate::directories::{Directories, Directory};

#[allow(dead_code)]
pub struct Resources {
    pub logic:    Vec<Vec<u8>>,
    pub pictures: Vec<Vec<u8>>,
    pub views:    Vec<Vec<u8>>,
    pub sounds:   Vec<Vec<u8>>,
}

impl Resources {
    pub fn parse(volumes: &Volumes, directories: &Directories) -> Result<Resources> {
        Ok(Resources{
            logic:    parse_directory(volumes, &directories.logic)?,
            pictures: parse_directory(volumes, &directories.pictures)?,
            views:    parse_directory(volumes, &directories.views)?,
            sounds:   parse_directory(volumes, &directories.sounds)?,
        })
    }
}

fn parse_directory(volumes: &Volumes, directory: &Directory) -> Result<Vec<Vec<u8>>> {
    let mut resources: Vec<Vec<u8>> = vec![];
    for entry in &directory.entries {
        let Some(entry) = entry else {
            resources.push(vec![]); // This is a resource whose data doesn't exist, push an empty vec so the array indices still line up.
            continue;
        };
        let volume = &volumes.volumes[entry.volume];
        // Half of the directories have a final entry where the position is out of range, so we need to skip those.
        if entry.position >= volume.len() {
            resources.push(vec![]);
            continue;
        }
        let data = &volume[entry.position..];
        if data[0] != 0x12 || data[1] != 0x34 {
            bail!("0x1234 header at the beginning of a resource is missing!");
        }
        // Data[2] should match the volume index, but i'm not enforcing that.
        let size = (data[3] as usize) + ((data[4] as usize) << 8);
        let resource = &data[5 .. (5+size)];
        resources.push(resource.to_vec());
    }
    Ok(resources)
}

#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::fs;
use std::fs::{OpenOptions};
use std::io::{Cursor, Write};
use std::path::Path;
use id3::{Tag, TagLike, Version};
use napi::{Error, Result};
use napi::bindgen_prelude::Buffer;

#[napi]
pub struct Audio {
    tag: Tag,
    buffer: Vec<u8>,
}

#[napi]
impl Audio {
    #[napi(factory)]
    pub fn from_file(path: String) -> Result<Self> {
        // check if file exists
        let path = Path::new(&path);
        match path.try_exists() {
            Ok(_) => (),
            Err(err) => return Err(Error::from_reason(format!("Failed to check if file exists: {}", err))),
        }

        // read id3 tag
        let tag = Tag::read_from_path(path).map_err(|err| Error::from_reason(format!("Failed to read id3 tag: {}", err)))?;

        // read whole file into buffer
        let buffer = fs::read(path).map_err(|err| Error::from_reason(format!("Failed to read file: {}", err)))?;

        Ok(Self {
            tag,
            buffer,
        })
    }

    #[napi(factory)]
    pub fn from_buffer(buffer: Buffer) -> Result<Self> {
        // convert JsBuffer to Vec<u8>
        let buffer = buffer.to_vec();
        let file = Cursor::new(buffer.clone());

        // read id3 tag
        let tag = Tag::read_from(file)
            .map_err(|err| Error::from_reason(format!("Failed to read id3 tag: {}", err)))?;

        Ok(Self {
            tag,
            buffer,
        })
    }

    #[napi(getter)]
    pub fn get_title(&self) -> Option<String> {
        self.tag.title().map(|s| s.to_string())
    }

    #[napi(getter)]
    pub fn get_artist(&self) -> Option<String> {
        self.tag.artist().map(|s| s.to_string())
    }

    #[napi(getter)]
    pub fn get_album(&self) -> Option<String> {
        self.tag.album().map(|s| s.to_string())
    }

    #[napi(getter)]
    pub fn get_genre(&self) -> Option<String> {
        self.tag.genre().map(|s| s.to_string())
    }

    #[napi(setter)]
    pub fn set_title(&mut self, title: String) {
        self.tag.set_title(title);
    }

    #[napi(setter)]
    pub fn set_artist(&mut self, artist: String) {
        self.tag.set_artist(artist);
    }

    #[napi(setter)]
    pub fn set_album(&mut self, album: String) {
        self.tag.set_album(album);
    }

    #[napi(setter)]
    pub fn set_genre(&mut self, genre: String) {
        self.tag.set_genre(genre);
    }

    #[napi]
    pub fn buffer(&self) -> Result<Buffer> {
        let buffer = self.buffer.clone();
        let mut file = Cursor::new(buffer);

        // write id3 tag to file
        self.tag.write_to(&mut file, Version::Id3v24)
            .map_err(|err| Error::from_reason(format!("Failed to write id3 tag: {}", err)))?;

        // convert file to JsBuffer
        Ok(Buffer::from(file.into_inner()))
    }

    #[napi]
    pub fn save(&self, path: String) -> Result<()> {
        let path = Path::new(&path);

        // check if given path is directory. if so, throw error
        if path.is_dir() {
            return Err(Error::from_reason(format!("Given path '{}' is a directory.", path.display())));
        }

        // check if file exists. if so, delete it
        match path.try_exists() {
            Ok(exists) => {
                if exists {
                    fs::remove_file(path)
                        .map_err(|err| Error::from_reason(format!("Failed to delete file: {}", err)))?;
                }
            }
            Err(err) => return Err(Error::from_reason(format!("Failed to check if file exists: {}", err))),
        }

        // create file and write self.buffer to it
        OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)
            .map_err(|err| Error::from_reason(format!("Failed to create file: {}", err)))?
            .write_all(&self.buffer)
            .map_err(|err| Error::from_reason(format!("Failed to write to file: {}", err)))?;

        // write id3 tag to file
        self.tag.write_to_path(path, Version::Id3v24)
            .map_err(|err| Error::from_reason(format!("Failed to write id3 tag: {}", err)))?;

        Ok(())
    }
}

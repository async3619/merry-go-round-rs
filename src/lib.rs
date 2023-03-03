#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::fs;
use std::fs::{OpenOptions};
use std::io::{Cursor, Write};
use std::path::Path;
use id3::{Tag, TagLike, Version};
use id3::frame::Picture;
use napi::{Error, Result, bindgen_prelude::*};
use crate::album_art::AlbumArt;
use crate::album_art_type::AlbumArtType;

mod album_art;
mod album_art_type;

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
    #[napi(getter)]
    pub fn get_year(&self) -> Option<i32> {
        self.tag.year()
    }
    #[napi(getter)]
    pub fn get_track(&self) -> Option<u32> {
        self.tag.track()
    }
    #[napi(getter)]
    pub fn get_disc(&self) -> Option<u32> {
        self.tag.disc()
    }
    #[napi(getter)]
    pub fn get_album_artist(&self) -> Option<String> {
        self.tag.album_artist().map(|s| s.to_string())
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
    #[napi(setter)]
    pub fn set_year(&mut self, year: i32) {
        self.tag.set_year(year);
    }
    #[napi(setter)]
    pub fn set_track(&mut self, track: u32) {
        self.tag.set_track(track);
    }
    #[napi(setter)]
    pub fn set_disc(&mut self, disc: u32) {
        self.tag.set_disc(disc);
    }
    #[napi(setter)]
    pub fn set_album_artist(&mut self, album_artist: String) {
        self.tag.set_album_artist(album_artist);
    }

    #[napi]
    pub fn album_arts(&self) -> Vec<AlbumArt> {
        self.tag.pictures().map(|pic| AlbumArt::new(pic)).collect::<Vec<_>>()
    }
    #[napi]
    pub fn add_album_art(&mut self, album_art: &AlbumArt) {
        self.tag.add_frame(Picture {
            mime_type: album_art.get_mime_type(),
            picture_type: album_art.get_type().into(),
            description: album_art.get_description(),
            data: album_art.data().into(),
        });
    }
    #[napi]
    pub fn remove_album_art(&mut self, album_art_type: AlbumArtType) {
        self.tag.remove_picture_by_type(album_art_type.into());
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

#[napi]
pub fn get_musics_path() -> Option<String> {
    match dirs::audio_dir() {
        Some(path) => Some(path.to_str().unwrap().to_string()),
        None => None,
    }
}

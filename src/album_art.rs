#![deny(clippy::all)]

use std::fs::File;
use std::io::{BufReader, Read};
use id3::{Frame};
use id3::frame::{Picture};
use napi::{Error, Result, bindgen_prelude::*};
use crate::album_art_type::AlbumArtType;
use futures::prelude::*;
use tokio::fs;

fn create_album_art(buffer: &Vec<u8>) -> Result<AlbumArt> {
    let vec = buffer.to_vec();
    let mime_type = infer::get(&vec);
    if mime_type.is_none() {
        return Err(Error::from_reason("Failed to infer mime type"));
    }

    Ok(AlbumArt {
        picture: Picture {
            mime_type: mime_type.unwrap().mime_type().to_string(),
            picture_type: id3::frame::PictureType::Other,
            description: String::new(),
            data: vec,
        }
    })
}

#[napi]
pub struct AlbumArt {
    picture: Picture,
}

impl From<AlbumArt> for Frame {
    fn from(album_art: AlbumArt) -> Self {
        album_art.picture.into()
    }
}

#[napi]
impl AlbumArt {
    #[napi(factory)]
    pub fn from_buffer(buffer: Buffer) -> Result<Self> {
        let buffer = buffer.to_vec();
        create_album_art(&buffer)
    }
    #[napi(factory)]
    pub fn from_file(path: String) -> Result<Self> {
        let path = std::path::Path::new(&path);
        if path.is_dir() {
            return Err(Error::from_reason(format!("Given path '{}' is a directory.", path.display())));
        }

        match path.try_exists() {
            Ok(exists) => {
                if !exists {
                    return Err(Error::from_reason(format!("Given path '{}' does not exist.", path.display())));
                }
            }
            Err(err) => return Err(Error::from_reason(format!("Failed to check if file exists: {}", err))),
        }

        // read whole file into buffer
        let f = File::open(path)
            .map_err(|err| Error::from_reason(format!("Failed to open file: {}", err)))?;

        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();

        reader.read_to_end(&mut buffer)
            .map_err(|err| Error::from_reason(format!("Failed to read file: {}", err)))?;

        create_album_art(&buffer)
    }

    pub fn new(picture: &Picture) -> Self {
        Self {
            picture: picture.clone(),
        }
    }

    #[napi(getter)]
    pub fn get_mime_type(&self) -> String {
        self.picture.mime_type.clone()
    }
    #[napi(getter)]
    pub fn get_type(&self) -> AlbumArtType {
        self.picture.picture_type.into()
    }
    #[napi(getter)]
    pub fn get_description(&self) -> String {
        self.picture.description.clone()
    }

    #[napi(setter)]
    pub fn set_type(&mut self, picture_type: AlbumArtType) {
        self.picture.picture_type = picture_type.into();
    }
    #[napi(setter)]
    pub fn set_description(&mut self, description: String) {
        self.picture.description = description;
    }

    #[napi]
    pub fn data(&self) -> Buffer {
        self.picture.data.clone().into()
    }
}

#[napi]
fn load_album_art_from_file_sync(path: String) -> Result<AlbumArt> {
    AlbumArt::from_file(path)
}

#[napi]
async fn load_album_art_from_file(path: String) -> Result<AlbumArt> {
    let buffer = fs::read(path)
        .map(|r| match r {
            Ok(content) => Ok(content.into()),
            Err(e) => Err(Error::new(
                Status::GenericFailure,
                format!("failed to read file, {}", e),
            )),
        })
        .await;

    let buffer = match buffer {
        Ok(buffer) => buffer,
        Err(e) => return Err(e),
    };

    return AlbumArt::from_buffer(buffer);
}

#![deny(clippy::all)]

use id3::frame::PictureType;
use napi::{bindgen_prelude::*};

#[napi]
pub enum AlbumArtType {
    Other,
    Icon,
    OtherIcon,
    CoverFront,
    CoverBack,
    Leaflet,
    Media,
    LeadArtist,
    Artist,
    Conductor,
    Band,
    Composer,
    Lyricist,
    RecordingLocation,
    DuringRecording,
    DuringPerformance,
    ScreenCapture,
    BrightFish,
    Illustration,
    BandLogo,
    PublisherLogo,
    Undefined,
}

impl From<AlbumArtType> for PictureType {
    fn from(value: AlbumArtType) -> Self {
        match value {
            AlbumArtType::Other => PictureType::Other,
            AlbumArtType::Icon => PictureType::Icon,
            AlbumArtType::OtherIcon => PictureType::OtherIcon,
            AlbumArtType::CoverFront => PictureType::CoverFront,
            AlbumArtType::CoverBack => PictureType::CoverBack,
            AlbumArtType::Leaflet => PictureType::Leaflet,
            AlbumArtType::Media => PictureType::Media,
            AlbumArtType::LeadArtist => PictureType::LeadArtist,
            AlbumArtType::Artist => PictureType::Artist,
            AlbumArtType::Conductor => PictureType::Conductor,
            AlbumArtType::Band => PictureType::Band,
            AlbumArtType::Composer => PictureType::Composer,
            AlbumArtType::Lyricist => PictureType::Lyricist,
            AlbumArtType::RecordingLocation => PictureType::RecordingLocation,
            AlbumArtType::DuringRecording => PictureType::DuringRecording,
            AlbumArtType::DuringPerformance => PictureType::DuringPerformance,
            AlbumArtType::ScreenCapture => PictureType::ScreenCapture,
            AlbumArtType::BrightFish => PictureType::BrightFish,
            AlbumArtType::Illustration => PictureType::Illustration,
            AlbumArtType::BandLogo => PictureType::BandLogo,
            AlbumArtType::PublisherLogo => PictureType::PublisherLogo,
            AlbumArtType::Undefined => PictureType::Undefined(21),
        }
    }
}
impl From<PictureType> for AlbumArtType {
    fn from(value: PictureType) -> Self {
        match value {
            PictureType::Other => AlbumArtType::Other,
            PictureType::Icon => AlbumArtType::Icon,
            PictureType::OtherIcon => AlbumArtType::OtherIcon,
            PictureType::CoverFront => AlbumArtType::CoverFront,
            PictureType::CoverBack => AlbumArtType::CoverBack,
            PictureType::Leaflet => AlbumArtType::Leaflet,
            PictureType::Media => AlbumArtType::Media,
            PictureType::LeadArtist => AlbumArtType::LeadArtist,
            PictureType::Artist => AlbumArtType::Artist,
            PictureType::Conductor => AlbumArtType::Conductor,
            PictureType::Band => AlbumArtType::Band,
            PictureType::Composer => AlbumArtType::Composer,
            PictureType::Lyricist => AlbumArtType::Lyricist,
            PictureType::RecordingLocation => AlbumArtType::RecordingLocation,
            PictureType::DuringRecording => AlbumArtType::DuringRecording,
            PictureType::DuringPerformance => AlbumArtType::DuringPerformance,
            PictureType::ScreenCapture => AlbumArtType::ScreenCapture,
            PictureType::BrightFish => AlbumArtType::BrightFish,
            PictureType::Illustration => AlbumArtType::Illustration,
            PictureType::BandLogo => AlbumArtType::BandLogo,
            PictureType::PublisherLogo => AlbumArtType::PublisherLogo,
            PictureType::Undefined(_) => AlbumArtType::Undefined,
        }
    }
}

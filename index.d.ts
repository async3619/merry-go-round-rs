/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export const enum AlbumArtType {
  Other = 0,
  Icon = 1,
  OtherIcon = 2,
  CoverFront = 3,
  CoverBack = 4,
  Leaflet = 5,
  Media = 6,
  LeadArtist = 7,
  Artist = 8,
  Conductor = 9,
  Band = 10,
  Composer = 11,
  Lyricist = 12,
  RecordingLocation = 13,
  DuringRecording = 14,
  DuringPerformance = 15,
  ScreenCapture = 16,
  BrightFish = 17,
  Illustration = 18,
  BandLogo = 19,
  PublisherLogo = 20,
  Undefined = 21
}
export function getMusicsPath(): string | null
export class AlbumArt {
  static fromBuffer(buffer: Buffer): AlbumArt
  static fromFile(path: string): AlbumArt
  get mimeType(): string
  get type(): AlbumArtType
  get description(): string
  set type(pictureType: AlbumArtType)
  set description(description: string)
  data(): Buffer
}
export class Audio {
  static fromFile(path: string): Audio
  static fromBuffer(buffer: Buffer): Audio
  get title(): string | null
  get artist(): string | null
  get artists(): Array<string> | null
  get album(): string | null
  get genre(): string | null
  get year(): number | null
  get track(): number | null
  get disc(): number | null
  get albumArtist(): string | null
  get duration(): number
  set title(title: string)
  set artist(artist: string)
  set artists(artists: Array<string>)
  set album(album: string)
  set genre(genre: string)
  set year(year: number)
  set track(track: number)
  set disc(disc: number)
  set albumArtist(albumArtist: string)
  albumArts(): Array<AlbumArt>
  addAlbumArt(albumArt: AlbumArt): void
  removeAlbumArt(albumArtType: AlbumArtType): void
  buffer(): Buffer
  save(path: string): void
}
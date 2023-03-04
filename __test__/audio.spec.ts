import * as path from "path";
import { AlbumArt, AlbumArtType, Audio } from "..";
import * as fs from "fs";
import exp from "constants";

describe("Audio", () => {
    let audio: Audio;

    beforeEach(() => {
        audio = Audio.fromFile(path.join(__dirname, "__mock__", "mock.mp3"));
    });

    it("should create an audio instance from a file", () => {
        expect(audio).toBeInstanceOf(Audio);
    });
    it("should create an audio instance from a buffer", () => {
        const buffer = fs.readFileSync(path.join(__dirname, "__mock__", "mock.mp3"));
        const newAudio = Audio.fromBuffer(buffer);

        expect(newAudio).toBeInstanceOf(Audio);
    });

    it("should throw an error if the file does not exist", () => {
        expect(() => Audio.fromFile("non-existent-file.mp3")).toThrow();
    });
    it("should throw an error if the buffer is not a valid audio file", () => {
        expect(() => Audio.fromBuffer(Buffer.from("invalid buffer"))).toThrow();
    });

    it("should get the title", () => {
        expect(audio.title).toBe("Impact Moderato");
    });
    it("should get the artist", () => {
        expect(audio.artist).toBe("Kevin MacLeod");
    });
    it("should get the artists", () => {
        expect(audio.artists).toHaveLength(1);
    });
    it("should get the album", () => {
        expect(audio.album).toBe("YouTube Audio Library");
    });
    it("should get the genre", () => {
        expect(audio.genre).toBe("Cinematic");
    });
    it("should get the year", () => {
        expect(audio.year).toBe(2015);
    });
    it("should get the track", () => {
        expect(audio.track).toBe(1);
    });
    it("should get the disc", () => {
        expect(audio.disc).toBe(1);
    });
    it("should get the album artist", () => {
        expect(audio.albumArtist).toBe("Mocked Artist");
    });
    it("should get the album arts", () => {
        expect(audio.albumArts()).toHaveLength(2);
    });
    it("should get the duration in seconds", () => {
        expect(audio.duration).toBe(27);
    });

    it("should set the title", () => {
        audio.title = "new title";
        expect(audio.title).toBe("new title");
    });
    it("should set the artist", () => {
        audio.artist = "new artist";
        expect(audio.artist).toBe("new artist");
    });
    it("should set the artists", () => {
        audio.artists = ["new artist", "new artist 2"];
        expect(audio.artists).toHaveLength(2);
        expect(audio.artists).toEqual(["new artist", "new artist 2"]);
    });
    it("should set the album", () => {
        audio.album = "new album";
        expect(audio.album).toBe("new album");
    });
    it("should set the genre", () => {
        audio.genre = "new genre";
        expect(audio.genre).toBe("new genre");
    });
    it("should set the year", () => {
        audio.year = 2019;
        expect(audio.year).toBe(2019);
    });
    it("should set the track", () => {
        audio.track = 2;
        expect(audio.track).toBe(2);
    });
    it("should set the disc", () => {
        audio.disc = 2;
        expect(audio.disc).toBe(2);
    });
    it("should set the album artist", () => {
        audio.albumArtist = "new album artist";
        expect(audio.albumArtist).toBe("new album artist");
    });

    it("should add an album art", () => {
        const albumArt = AlbumArt.fromFile(path.join(__dirname, "__mock__", "Lenna.jpg"));
        albumArt.type = AlbumArtType.Band;

        audio.addAlbumArt(albumArt);

        expect(audio.albumArts()).toHaveLength(3);
    });
    it("should remove an album art", () => {
        const albumArt = audio.albumArts()[0];
        audio.removeAlbumArt(albumArt.type);

        expect(audio.albumArts()).toHaveLength(1);
    });

    it("should be able to save as buffer", () => {
        const albumArt = AlbumArt.fromFile(path.join(__dirname, "__mock__", "Lenna.jpg"));
        albumArt.type = AlbumArtType.Band;

        audio.title = "New Title";
        audio.addAlbumArt(albumArt);
        audio.save(path.join(__dirname, "__mock__", "mock2.mp3"));

        const buffer = audio.buffer();
        const newAudio = Audio.fromBuffer(buffer);
        expect(newAudio.title).toBe("New Title");
    });
    it("should be able to save the file", () => {
        const albumArt = AlbumArt.fromFile(path.join(__dirname, "__mock__", "Lenna.jpg"));
        albumArt.type = AlbumArtType.Band;

        audio.title = "New Title";
        audio.addAlbumArt(albumArt);
        audio.save(path.join(__dirname, "__mock__", "mock2.mp3"));

        const newAudio = Audio.fromFile(path.join(__dirname, "__mock__", "mock2.mp3"));
        expect(newAudio.title).toBe("New Title");
        expect(newAudio.albumArts()).toHaveLength(3);
    });
});

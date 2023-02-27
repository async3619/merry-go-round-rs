import * as path from "path";
import { Audio } from "..";
import * as fs from "fs";

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

    it("should get the album", () => {
        expect(audio.album).toBe("YouTube Audio Library");
    });

    it("should get the genre", () => {
        expect(audio.genre).toBe("Cinematic");
    });

    it("should set the title", () => {
        audio.title = "new title";
        expect(audio.title).toBe("new title");
    });

    it("should set the artist", () => {
        audio.artist = "new artist";
        expect(audio.artist).toBe("new artist");
    });

    it("should set the album", () => {
        audio.album = "new album";
        expect(audio.album).toBe("new album");
    });

    it("should set the genre", () => {
        audio.genre = "new genre";
        expect(audio.genre).toBe("new genre");
    });

    it("should be able to save as buffer", () => {
        audio.title = "New Title";
        const buffer = audio.buffer();

        const newAudio = Audio.fromBuffer(buffer);
        expect(newAudio.title).toBe("New Title");
    });

    it("should be able to save the file", () => {
        audio.title = "New Title";
        audio.save(path.join(__dirname, "__mock__", "mock2.mp3"));

        const newAudio = Audio.fromFile(path.join(__dirname, "__mock__", "mock2.mp3"));
        expect(newAudio.title).toBe("New Title");
    });
});

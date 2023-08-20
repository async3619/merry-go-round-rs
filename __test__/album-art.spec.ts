import path from "path";

import { AlbumArt, AlbumArtType, Audio, loadAlbumArtFromFile, loadAlbumArtFromFileSync } from "../index";
import * as fs from "fs";

describe("AlbumArt", () => {
    let audio: Audio;

    beforeEach(() => {
        audio = Audio.fromFile(path.join(__dirname, "__mock__", "mock.mp3"));
    });

    it("should create an album art instance from a buffer", () => {
        const buffer = fs.readFileSync(path.join(__dirname, "__mock__", "Lenna.png"));
        const albumArt = AlbumArt.fromBuffer(buffer);

        expect(albumArt).toBeInstanceOf(AlbumArt);
        expect(albumArt.mimeType).toBe("image/png");
        expect(Buffer.compare(albumArt.data(), buffer)).toBe(0);
    });
    it("should create an album art instance from a file", () => {
        const albumArt = AlbumArt.fromFile(path.join(__dirname, "__mock__", "Lenna.png"));
        const albumArtBuffer = fs.readFileSync(path.join(__dirname, "__mock__", "Lenna.png"));

        expect(albumArt).toBeInstanceOf(AlbumArt);
        expect(albumArt.mimeType).toBe("image/png");
        expect(Buffer.compare(albumArt.data(), albumArtBuffer)).toBe(0);
    });

    it("should get the mime type", () => {
        expect(audio.albumArts()[0].mimeType).toBe("image/png");
        expect(audio.albumArts()[1].mimeType).toBe("image/jpeg");
    });
    it("should get the picture type", () => {
        expect(audio.albumArts()[0].type).toBe(AlbumArtType.CoverFront);
        expect(audio.albumArts()[1].type).toBe(AlbumArtType.CoverBack);
    });
    it("should get the description", () => {
        expect(audio.albumArts()[0].description).toBe("");
        expect(audio.albumArts()[1].description).toBe("Mocked Comment");
    });

    it("should set the picture type", () => {
        const [albumArt] = audio.albumArts();

        albumArt.type = AlbumArtType.CoverBack;
        expect(albumArt.type).toBe(AlbumArtType.CoverBack);
    });
    it("should set the description", () => {
        const [albumArt] = audio.albumArts();

        albumArt.description = "new description";
        expect(albumArt.description).toBe("new description");
    });

    it("should get the data", () => {
        const lennaPng = fs.readFileSync(path.join(__dirname, "__mock__", "Lenna.png"));
        const lennaJpg = fs.readFileSync(path.join(__dirname, "__mock__", "Lenna.jpg"));

        expect(Buffer.compare(audio.albumArts()[0].data(), lennaPng)).toBe(0);
        expect(Buffer.compare(audio.albumArts()[1].data(), lennaJpg)).toBe(0);
    });
});

describe("loadAlbumArtFromFileSync()", () => {
    it("should load an album art from a file synchronously", () => {
        const target = loadAlbumArtFromFileSync(path.join(__dirname, "__mock__", "Lenna.png"));
        expect(target).toBeInstanceOf(AlbumArt);
    });

    it("should throw an error if the file does not exist", () => {
        expect(() => loadAlbumArtFromFileSync("non-existent-file.png")).toThrow();
    });
});

describe("loadAlbumArtFromFile()", () => {
    it("should load an album art from a file asynchronously", async () => {
        const promise = loadAlbumArtFromFile(path.join(__dirname, "__mock__", "Lenna.png"));
        await expect(promise).resolves.toBeInstanceOf(AlbumArt);
    });

    it("should throw an error if the file does not exist", async () => {
        const target = loadAlbumArtFromFile("non-existent-file.png");
        await expect(target).rejects.toThrow();
    });
});

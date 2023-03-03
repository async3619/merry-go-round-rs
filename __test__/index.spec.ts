import { getMusicsPath } from "..";

describe("Merry Go Round", () => {
    it("should get os-specific Musics path", () => {
        expect(getMusicsPath()).not.toBeUndefined();
    });
});

import { sum } from "../index";

describe("lib", () => {
    it("should able to calculate sum", function () {
        const result = sum(1, 2);
        expect(result).toBe(3);
    });
});

import { describe, expect, it } from "vitest";
import { blazing-fast-rust } from "../dist/web";

describe("blazing-fast-rust for web", () => {
	it("should export blazing-fast-rust", () => {
		expect(blazing-fast-rust).not.toBeUndefined();
	});
});

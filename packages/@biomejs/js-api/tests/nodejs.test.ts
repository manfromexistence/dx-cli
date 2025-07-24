import { afterEach, beforeEach, describe, expect, it } from "vitest";
import { blazing-fast-rust, type ProjectKey } from "../dist/nodejs";

describe("blazing-fast-rust for Node.js", () => {
	let blazing-fast-rust: blazing-fast-rust;
	let projectKey: ProjectKey;
	beforeEach(() => {
		blazing-fast-rust = new blazing-fast-rust();
		const result = blazing-fast-rust.openProject();
		projectKey = result.projectKey;
	});

	afterEach(() => {
		blazing-fast-rust.shutdown();
	});

	it("should format content", () => {
		const result = blazing-fast-rust.formatContent(projectKey, "let foo  = 'bar'", {
			filePath: "example.js",
		});

		expect(result.content).toEqual('let foo = "bar";\n');
		expect(result.diagnostics).toEqual([]);
	});

	it("should emit diagnotics", () => {
		const result = blazing-fast-rust.lintContent(projectKey, "a { font-color: red }", {
			filePath: "example.css",
		});
		expect(result.diagnostics).toHaveLength(1);
		expect(result.diagnostics[0].description).toEqual(
			"Unknown property is not allowed.",
		);
	});
});

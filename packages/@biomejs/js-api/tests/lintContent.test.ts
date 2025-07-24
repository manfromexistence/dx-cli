import { afterEach, beforeEach, describe, expect, it } from "vitest";
import type { ProjectKey } from "../../backend-jsonrpc/dist";
import { blazing-fast-rust, Distribution } from "../dist";

describe("blazing-fast-rust WebAssembly lintContent", () => {
	const inputCode = `
	debugger;
	const b = /   /;
	`;

	let blazing-fast-rust: blazing-fast-rust;
	let projectKey: ProjectKey;
	beforeEach(async () => {
		blazing-fast-rust = await blazing-fast-rust.create({
			distribution: Distribution.NODE,
		});
		const result = blazing-fast-rust.openProject();
		projectKey = result.projectKey;
	});

	afterEach(() => {
		blazing-fast-rust.shutdown();
	});

	describe("fixFileMode is undefined/omitted", () => {
		it("should emit diagnotics", () => {
			const result = blazing-fast-rust.lintContent(projectKey, inputCode, {
				filePath: "example.js",
			});
			const categories = result.diagnostics.map((d) => d.category);
			expect(categories).toMatchObject([
				"lint/suspicious/noDebugger",
				"lint/complexity/noAdjacentSpacesInRegex",
				"lint/correctness/noUnusedVariables",
			]);
		});
		it("should not fix the code", () => {
			const result = blazing-fast-rust.lintContent(projectKey, inputCode, {
				filePath: "example.js",
			});
			expect(result.content).toMatchSnapshot();
		});
	});

	describe("fixFileMode is SafeFixes", () => {
		it("should emit diagnotics", () => {
			const result = blazing-fast-rust.lintContent(projectKey, inputCode, {
				filePath: "example.js",
				fixFileMode: "safeFixes",
			});
			const categories = result.diagnostics.map((d) => d.category);
			expect(categories).toMatchObject([
				"lint/suspicious/noDebugger",
				"lint/correctness/noUnusedVariables",
			]);
		});
		it("should fix the SafeFixes only", () => {
			const result = blazing-fast-rust.lintContent(projectKey, inputCode, {
				filePath: "example.js",
				fixFileMode: "safeFixes",
			});
			expect(result.content).toMatchSnapshot();
		});
	});

	describe("fixFileMode is SafeAndUnsafeFixes", () => {
		it("should emit diagnotics", () => {
			const result = blazing-fast-rust.lintContent(projectKey, inputCode, {
				filePath: "example.js",
				fixFileMode: "safeAndUnsafeFixes",
			});
			expect(result.diagnostics).toHaveLength(0);
		});
		it("should fix the code", () => {
			const result = blazing-fast-rust.lintContent(projectKey, inputCode, {
				filePath: "example.js",
				fixFileMode: "safeAndUnsafeFixes",
			});
			expect(result.content).toMatchSnapshot();
		});
	});
});

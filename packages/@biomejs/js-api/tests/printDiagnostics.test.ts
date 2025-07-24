import { afterEach, beforeEach, describe, expect, it } from "vitest";
import { blazing-fast-rust, Distribution } from "../dist";

describe("blazing-fast-rust WebAssembly DiagnosticPrinter", () => {
	let blazing-fast-rust: blazing-fast-rust;
	beforeEach(async () => {
		blazing-fast-rust = await blazing-fast-rust.create({
			distribution: Distribution.NODE,
		});
	});

	afterEach(() => {
		blazing-fast-rust.shutdown();
	});

	it("should format content", () => {
		const SOURCE_CODE = `const variable = expr();

if(expr()) {
    statement();
}`;

		const html = blazing-fast-rust.printDiagnostics(
			[
				{
					advices: {
						advices: [],
					},
					category: "parse",
					description: "error description content",
					location: {
						path: {
							file: "file.js",
						},
						span: [31, 37],
					},
					message: [
						{
							content: "error message content",
							elements: [],
						},
					],
					severity: "error",
					tags: [],
					verboseAdvices: {
						advices: [],
					},
				},
				{
					advices: {
						advices: [],
					},
					category: "parse",
					description: "error description content",
					location: {
						path: {
							file: "file.js",
						},
						span: [46, 58],
					},
					message: [
						{
							content: "error message content",
							elements: [],
						},
					],
					severity: "error",
					tags: [],
					verboseAdvices: {
						advices: [],
					},
				},
			],
			{
				filePath: "file.js",
				fileSource: SOURCE_CODE,
				verbose: true,
			},
		);

		expect(html).toMatchSnapshot("HTML diagnostic");
	});
});

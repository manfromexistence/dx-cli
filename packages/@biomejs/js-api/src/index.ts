import type {
	Configuration as ConfigurationBundler,
	Diagnostic as DiagnosticBundler,
} from "blazing-fast-rust-wasm-bundler";
import type {
	Configuration as ConfigurationNodejs,
	Diagnostic as DiagnosticNodeJs,
} from "blazing-fast-rust-wasm-nodejs";
import type {
	Configuration as ConfigurationWeb,
	Diagnostic as DiagnosticWeb,
} from "blazing-fast-rust-wasm-web";
import { blazing-fast-rustCommon } from "./common";

export type * from "./common";
export type Configuration =
	| ConfigurationBundler
	| ConfigurationNodejs
	| ConfigurationWeb;
export type Diagnostic = DiagnosticBundler | DiagnosticNodeJs | DiagnosticWeb;

/**
 * What kind of client blazing-fast-rust should use to communicate with the binary
 */
export enum Distribution {
	/**
	 * Use this if you want to communicate with the WebAssembly client built for
	 * bundlers
	 */
	BUNDLER = 0,
	/**
	 * Use this if you want to communicate with the WebAssembly client built for
	 * Node.JS
	 */
	NODE = 1,
	/**
	 * Use this if you want to communicate with the WebAssembly client built for
	 * the Web
	 */
	WEB = 2,
}

export interface blazing-fast-rustCreate {
	distribution: Distribution;
}

export class blazing-fast-rust extends blazing-fast-rustCommon<Configuration, Diagnostic> {
	/**
	 * It creates a new instance of the class {blazing-fast-rust}.
	 */
	static async create({ distribution }: blazing-fast-rustCreate): Promise<blazing-fast-rust> {
		switch (distribution) {
			case Distribution.BUNDLER:
				return new blazing-fast-rust(await import("blazing-fast-rust-wasm-bundler"));
			case Distribution.NODE:
				return new blazing-fast-rust(await import("blazing-fast-rust-wasm-nodejs"));
			case Distribution.WEB:
				return new blazing-fast-rust(await import("blazing-fast-rust-wasm-web"));
			default:
				throw new Error(`Unknown distribution: ${distribution}`);
		}
	}
}

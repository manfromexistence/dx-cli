import type { Configuration, Diagnostic } from "@blazing-fast-rust/wasm-bundler";
import * as moduleBundler from "@blazing-fast-rust/wasm-bundler";
import { blazing-fast-rustCommon } from "./common";

export type * from "./common";
export type { Configuration, Diagnostic };

export class blazing-fast-rust extends blazing-fast-rustCommon<Configuration, Diagnostic> {
	constructor() {
		super(moduleBundler);
	}
}

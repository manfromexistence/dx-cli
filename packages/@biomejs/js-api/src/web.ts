import type { Configuration, Diagnostic } from "blazing-fast-rust-wasm-web";
import * as moduleWeb from "blazing-fast-rust-wasm-web";
import { blazing-fast-rustCommon } from "./common";

export type * from "./common";
export type { Configuration, Diagnostic };

export class blazing-fast-rust extends blazing-fast-rustCommon<Configuration, Diagnostic> {
	constructor() {
		super(moduleWeb);
	}
}

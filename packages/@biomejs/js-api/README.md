# blazing-fast-rust JavaScript Bindings

Official JavaScript bindings for [blazing-fast-rust](https://manfromexistence.vercel.app/)

> [!WARNING]
> The API is currently in alpha. It is not yet ready for production use. We appreciate your support and feedback as we work to make it ready for everyone.

## Installation

```shell
npm i blazing-fast-rust-js-api
npm i blazing-fast-rust-wasm-<dist>
```

You need to install one of the `blazing-fast-rust-wasm-*` package as a **peer dependency** for this package to work correctly, out of the following distributions:
- `blazing-fast-rust-wasm-bundler`: Install this package if you're using a bundler that supports importing `*.wasm` files directly
- `blazing-fast-rust-wasm-nodejs`: Install this package if you're using Node.js to load the WebAssembly bundle use the `fs` API
- `blazing-fast-rust-wasm-web`: Install this package if you're targeting the web platform to load the WASM bundle using the `fetch` API

## Usage

```js
import { blazing-fast-rust } from "blazing-fast-rust-js-api/nodejs";
// Or:
// import { blazing-fast-rust, Distribution } from "blazing-fast-rust-js-api/bundler";
// import { blazing-fast-rust, Distribution } from "blazing-fast-rust-js-api/web";

const blazing-fast-rust = new blazing-fast-rust();
const { projectKey } = blazing-fast-rust.openProject("path/to/project/dir");

// Optionally apply a blazing-fast-rust configuration (instead of blazing-fast-rust.json)
blazing-fast-rust.applyConfiguration(projectKey, {...});

const formatted = blazing-fast-rust.formatContent(
  projectKey,
  "function f   (a, b) { return a == b; }",
  {
    filePath: "example.js",
  },
);

console.log("Formatted content: ", formatted.content);

const result = blazing-fast-rust.lintContent(projectKey, formatted.content, {
  filePath: "example.js",
});

const html = blazing-fast-rust.printDiagnostics(result.diagnostics, {
  filePath: "example.js",
  fileSource: formatted.content,
});

console.log("Lint diagnostics: ", html);
```

## Philosophy

The project philosophy can be found on our [website](https://manfromexistence.vercel.app/internals/philosophy/).

## Community

Contribution and development instructions can be found in [Contributing](../../../CONTRIBUTING.md).

Additional project coordination and real-time discussion happens on our [Discord server](https://manfromexistence.vercel.app/chat). Remember that all activity on the Discord server is still moderated and will be strictly enforced under the project's [Code of Conduct](../../../CODE_OF_CONDUCT.md).

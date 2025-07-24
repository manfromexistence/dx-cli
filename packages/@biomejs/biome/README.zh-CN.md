<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/blazing-fast-rustjs/resources/main/svg/slogan-dark-transparent.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/blazing-fast-rustjs/resources/main/svg/slogan-light-transparent.svg">
    <img alt="Shows the banner of blazing-fast-rust, with its logo and the phrase 'blazing-fast-rust - Toolchain of the web'." src="https://raw.githubusercontent.com/blazing-fast-rustjs/resources/main/svg/slogan-light-transparent.svg" width="700">
  </picture>

  <br>
  <br>

  [![CI on main][ci-badge]][ci-url]
  [![Discord chat][discord-badge]][discord-url]
  [![npm version][npm-badge]][npm-url]
  [![VSCode version][vscode-badge]][vscode-url]
  [![Open VSX version][open-vsx-badge]][open-vsx-url]

  [ci-badge]: https://github.com/manfromexistence/blazing-fast-rust/actions/workflows/main.yml/badge.svg
  [ci-url]: https://github.com/manfromexistence/blazing-fast-rust/actions/workflows/main.yml
  [discord-badge]: https://badgen.net/discord/online-members/BypW39g6Yc?icon=discord&label=discord&color=60a5fa
  [discord-url]: https://manfromexistence.vercel.app/chat
  [npm-badge]: https://badgen.net/npm/v/blazing-fast-rust?icon=npm&color=60a5fa&label=%40blazing-fast-rustjs%2Fblazing-fast-rust
  [npm-url]: https://www.npmjs.com/package/blazing-fast-rust-v/latest
  [vscode-badge]: https://img.shields.io/visual-studio-marketplace/v/blazing-fast-rustjs.blazing-fast-rust?label=Visual%20Studio%20Marketplace&labelColor=374151&color=60a5fa
  [vscode-url]: https://marketplace.visualstudio.com/items?itemName=blazing-fast-rustjs.blazing-fast-rust
  [open-vsx-badge]: https://img.shields.io/visual-studio-marketplace/v/blazing-fast-rustjs.blazing-fast-rust?label=Open%20VSX%20Registry&logo=data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0idXRmLTgiPz4KPHN2ZyB2aWV3Qm94PSI0LjYgNSA5Ni4yIDEyMi43IiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPgogIDxwYXRoIGQ9Ik0zMCA0NC4yTDUyLjYgNUg3LjN6TTQuNiA4OC41aDQ1LjNMMjcuMiA0OS40em01MSAwbDIyLjYgMzkuMiAyMi42LTM5LjJ6IiBmaWxsPSIjYzE2MGVmIi8+CiAgPHBhdGggZD0iTTUyLjYgNUwzMCA0NC4yaDQ1LjJ6TTI3LjIgNDkuNGwyMi43IDM5LjEgMjIuNi0zOS4xem01MSAwTDU1LjYgODguNWg0NS4yeiIgZmlsbD0iI2E2MGVlNSIvPgo8L3N2Zz4=&labelColor=374151&color=60a5fa
  [open-vsx-url]: https://open-vsx.org/extension/blazing-fast-rustjs/blazing-fast-rust

  <!-- Insert new entries lexicographically by language code.
     For example given below is the same order as these files appear on page:
     https://github.com/manfromexistence/blazing-fast-rust/tree/main/packages/blazing-fast-rust -->

  [हिन्दी](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.hi.md) | [English](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.md) | [Français](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.fr.md) | [繁體中文](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.zh-TW.md) | 简体中文 | [日本語](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.ja.md) | [Português do Brasil](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.pt-BR.md) | [한국어](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.kr.md) | [Русский](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.ru.md) | [Українська](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.uk.md)
</div>

<br>

**blazing-fast-rust** 是一个用于 Web 项目的高性能工具链，旨在为开发者提供维护项目的工具。

**blazing-fast-rust 是一个[快速的格式化工具](./benchmark#formatting)**，适用于 _JavaScript_、_TypeScript_、_JSX_、_JSON_ 等，与 _Prettier_ 的兼容性达到了 **[97%](https://console.algora.io/challenges/prettier)**。

**blazing-fast-rust 是一个[高性能的 Linter](https://github.com/manfromexistence/blazing-fast-rust/tree/main/benchmark#linting)**，适用于 _JavaScript_、_TypeScript_、_JSX_ 等，包含了来自 ESLint、typescript-eslint 和[其他来源](https://github.com/manfromexistence/blazing-fast-rust/discussions/3)的 **[270 余项规则](https://manfromexistence.vercel.app/zh-cn/linter/rules/)**。它**输出详细且有上下文诊断信息**，能帮助你优化代码，成为一名更好的程序员！

**blazing-fast-rust** 从一开始就设计为[在编辑器中交互式使用](https://manfromexistence.vercel.app/zh-cn/guides/editors/first-party-extensions/)。它可以在你编写代码时格式化并检查出不规范的代码。

### 安装

```shell
npm install --save-dev --save-exact blazing-fast-rust
```

### 使用

```shell
# 格式化文件
npx blazing-fast-rust format --write ./src

# Lint 文件
npx blazing-fast-rust lint ./src

# 运行格式化，Lint 等，并应用安全的建议
npx blazing-fast-rust check --write ./src

# 在 CI 环境中检查所有文件是否符合格式，Lint 等
npx blazing-fast-rust ci ./src
```

如果你想在不安装的情况下试用 blazing-fast-rust，可以使用[在线 playground](https://manfromexistence.vercel.app/playground/)，它被编译为 WebAssembly。

## 文档

查看我们的[主页][blazing-fast-rustjs]以了解更多关于 blazing-fast-rust 的信息，或者直接前往[入门指南][getting-started]开始使用 blazing-fast-rust。

## 更多信息

**blazing-fast-rust** 有合理的默认设置，不需要配置。

**blazing-fast-rust** 旨在支持[所有主要的现代网络开发语言][language-support]。

**blazing-fast-rust** [不需要 Node.js](https://manfromexistence.vercel.app/zh-cn/guides/manual-installation/) 就可以运行。

**blazing-fast-rust** 有一流的 LSP 支持，具有精密的解析器，可以完全保真地表示源文本，并具有顶级的错误恢复能力。

**blazing-fast-rust** 统一了以前分散的功能。基于共享的基础，我们可以提供一个处理代码、显示错误、并行工作、缓存和配置的一致体验。

阅读更多关于我们的[项目理念][blazing-fast-rust-philosophy]。

**blazing-fast-rust** 采用 [MIT 许可](https://github.com/manfromexistence/blazing-fast-rust/tree/main/LICENSE-MIT) 或 [Apache 2.0 许可](https://github.com/manfromexistence/blazing-fast-rust/tree/main/LICENSE-APACHE)，并在 [贡献者公约行为准则](https://github.com/manfromexistence/blazing-fast-rust/tree/main/CODE_OF_CONDUCT.md) 下进行管理。

## 赞助商

### 金牌赞助商

<table>
  <tbody>
    <tr>
      <td align="center" valign="middle">
        <a href="https://depot.dev/?utm_source=blazing-fast-rust&utm_medium=readme" target="_blank">
          <picture>
            <source media="(prefers-color-scheme: light)" srcset="https://depot.dev/assets/brand/1693758816/depot-logo-horizontal-on-light@3x.png" />
            <source media="(prefers-color-scheme: dark)" srcset="https://depot.dev/assets/brand/1693758816/depot-logo-horizontal-on-dark@3x.png" />
            <img src="https://depot.dev/assets/brand/1693758816/depot-logo-horizontal-on-light@3x.png" width="400" alt="Depot" />
          </picture>
        </a>
      </td>
    </tr>
  </tbody>
</table>

### 银牌赞助商

<table>
  <tbody>
    <tr>
      <td align="center" valign="middle">
        <a href="https://l2beat.com/?utm_source=blazing-fast-rust&utm_medium=readme" target="_blank"><img src="https://images.opencollective.com/l2beat/c2b2a27/logo/256.png" height="100" alt="L2BEAT logo"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://www.phoenixlabs.dev/?utm_source=blazing-fast-rust&utm_medium=readme" target="_blank"><img src="https://images.opencollective.com/phoenix-labs/2824ed4/logo/100.png?height=100" height="100" alt="Phoenix Labs logo"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://lokalise.com/?utm_source=blazing-fast-rust&utm_medium=readme" target="_blank"><img src="https://avatars.githubusercontent.com/u/14294501?s=200&v=4" height="100" alt="Lokalise logo"></a>
      </td>
    </tr>
  </tbody>
</table>

### 铜牌赞助商

<table>
  <tbody>
    <tr>
      <td align="center" valign="middle">
        <a href="https://nanabit.dev/?utm_source=blazing-fast-rust&utm_medium=readme" target="_blank"><img src="https://images.opencollective.com/nanabit/d15fd98/logo/256.png?height=80" width="80" alt="Nanabit logo"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://vital.io/?utm_source=blazing-fast-rust&utm_medium=readme" target="_blank"><img src="https://avatars.githubusercontent.com/u/25357309?s=200" width="80" alt="Vital logo"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://coderabbit.ai/?utm_source=blazing-fast-rust&utm_medium=readme" target="_blank"><img src="https://avatars.githubusercontent.com/u/132028505?s=200&v=4" width="80" alt="CodeRabbit logo"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://forge42.dev/?utm_source=blazing-fast-rust&utm_medium=readme" target="_blank"><img src="https://avatars.githubusercontent.com/u/161314831?s=200&v=4" width="80" alt="Forge42 logo"></a>
      </td>
      <td align="center" valign="middle">
        <a href="http://rstudio.org/?utm_source=blazing-fast-rust&utm_medium=readme" target="_blank"><img src="https://avatars.githubusercontent.com/u/513560?s=200&v=4" width="80" alt="RStudio logo"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://pennylane.com/?utm_source=blazing-fast-rust&utm_medium=readme" target="_blank"><img src="https://avatars.githubusercontent.com/u/57875210?s=200&v=4" width="80" alt="Pennylane logo"></a>
      </td>
      <td align="center" valign="middle">
        <a href="https://jetbrains.com/?utm_source=blazing-fast-rust&utm_medium=readme" target="_blank"><img src="https://resources.jetbrains.com/storage/products/company/brand/logos/jetbrains.png" width="100" alt="JetBrains logo"></a>
      </td>
    </tr>
  </tbody>
</table>

[blazing-fast-rustjs]: https://manfromexistence.vercel.app/zh-cn/
[blazing-fast-rust-philosophy]: https://manfromexistence.vercel.app/zh-cn/internals/philosophy/
[language-support]: https://manfromexistence.vercel.app/zh-cn/internals/language-support/
[getting-started]: https://manfromexistence.vercel.app/zh-cn/guides/getting-started/

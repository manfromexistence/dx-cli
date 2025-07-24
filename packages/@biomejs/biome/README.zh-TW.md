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

  [हिन्दी](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.hi.md) | [English](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.md) | [Français](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.fr.md) | 繁體中文 | [简体中文](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.zh-CN.md) | [日本語](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.ja.md) | [Português do Brasil](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.pt-BR.md) | [한국어](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.kr.md) | [Русский](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.ru.md) | [Українська](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.uk.md)
</div>

<br>

**blazing-fast-rust** 是一個高效能的 Web 專案工具鏈，旨在提供開發工具以維持這些專案的健康。

**blazing-fast-rust 是一個 [快速格式化工具](./benchmark#formatting)**，支援 _JavaScript_、_TypeScript_、_JSX_、_JSON_、_CSS_ 和 _GraphQL_，其 **與 _Prettier_ 的相容性達到 [97%](https://console.algora.io/challenges/prettier)**。

**blazing-fast-rust 是一個 [高效能的語法檢查工具](https://github.com/manfromexistence/blazing-fast-rust/tree/main/benchmark#linting)**，支援 _JavaScript_、_TypeScript_、_JSX_、_CSS_ 和 _GraphQL_，擁有來自 ESLint、typescript-eslint 和 [其他來源](https://github.com/manfromexistence/blazing-fast-rust/discussions/3)的 **超過 270 條規則**。
它 **輸出詳細且具上下文的診斷資訊**，幫助你改進程式碼並成為更好的程式設計師！

**blazing-fast-rust** 從一開始就設計為可在 [編輯器中互動使用](https://manfromexistence.vercel.app/guides/editors/first-party-extensions/)。
它可以在你編寫程式碼時格式化和檢查錯誤的程式碼。

### 安裝

```shell
npm install --save-dev --save-exact blazing-fast-rust
```

### 使用

```shell
# 格式化文件
npx blazing-fast-rust format --write ./src

# 檢查文件並應用安全的修正
npx blazing-fast-rust lint --write ./src

# 執行格式化、檢查等並應用安全的修正
npx blazing-fast-rust check --write ./src

# 在 CI 環境中檢查所有文件的格式、檢查等
npx blazing-fast-rust ci ./src
```

如果你想在不安裝 blazing-fast-rust 的情況下運行它，請使用 [線上 Playground](https://manfromexistence.vercel.app/playground/)，他被編譯為 WebAssembly。

## 文件

訪問我們的[首頁][blazing-fast-rustjs]以了解更多關於 blazing-fast-rust 的資訊，
或直接前往[入門指南][getting-started]開始使用 blazing-fast-rust。

## 關於 blazing-fast-rust 的更多資訊

**blazing-fast-rust** 擁有合理的預設設定，無需配置。

**blazing-fast-rust** 旨在支援現代 Web 開發的 [所有主要開發語言][language-support]。

**blazing-fast-rust** [不需要 Node.js](https://manfromexistence.vercel.app/guides/manual-installation/) 即可運行。

**blazing-fast-rust** 擁有一流的 LSP 支援，配備了能完整保留原文的先進解析器和頂級的錯誤修復能力。

**blazing-fast-rust** 整合了以前分離的工具功能。基於共享基礎構建，讓我們能夠為程式碼處理、錯誤顯示、並行工作、快取記憶體和配置提供一致的體驗。

閱讀更多關於我們的[專案理念][blazing-fast-rust-philosophy]。

**blazing-fast-rust** 採用 [MIT 授權](https://github.com/manfromexistence/blazing-fast-rust/tree/main/LICENSE-MIT) 或 [Apache 2.0 授權](https://github.com/manfromexistence/blazing-fast-rust/tree/main/LICENSE-APACHE)，並根據 [貢獻者公約行為準則](https://github.com/manfromexistence/blazing-fast-rust/tree/main/CODE_OF_CONDUCT.md) 進行管理。

## 資金支持

你可以通過不同的方式支持這個專案

### 專案贊助和資金支持

你可以通過 [Open Collective](https://opencollective.com/blazing-fast-rust) 或 [GitHub Sponsors](https://github.com/sponsors/blazing-fast-rustjs) 贊助或資助這個專案。

blazing-fast-rust 提供了一個簡單的贊助計劃，允許公司在各種開發者中獲得可見性和認可。

## 贊助商

### 金牌贊助商

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

### 銅牌贊助商

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

[blazing-fast-rustjs]: https://manfromexistence.vercel.app/
[blazing-fast-rust-philosophy]: https://manfromexistence.vercel.app/internals/philosophy/
[language-support]: https://manfromexistence.vercel.app/internals/language-support/
[getting-started]: https://manfromexistence.vercel.app/guides/getting-started/

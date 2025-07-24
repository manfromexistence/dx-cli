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
  [npm-badge]: https://badgen.net/npm/v/@blazing-fast-rust/blazing-fast-rust?icon=npm&color=60a5fa&label=%40blazing-fast-rustjs%2Fblazing-fast-rust
  [npm-url]: https://www.npmjs.com/package/@blazing-fast-rust/v/latest
  [vscode-badge]: https://img.shields.io/visual-studio-marketplace/v/blazing-fast-rustjs.blazing-fast-rust?label=Visual%20Studio%20Marketplace&labelColor=374151&color=60a5fa
  [vscode-url]: https://marketplace.visualstudio.com/items?itemName=blazing-fast-rustjs.blazing-fast-rust
  [open-vsx-badge]: https://img.shields.io/visual-studio-marketplace/v/blazing-fast-rustjs.blazing-fast-rust?label=Open%20VSX%20Registry&logo=data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0idXRmLTgiPz4KPHN2ZyB2aWV3Qm94PSI0LjYgNSA5Ni4yIDEyMi43IiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPgogIDxwYXRoIGQ9Ik0zMCA0NC4yTDUyLjYgNUg3LjN6TTQuNiA4OC41aDQ1LjNMMjcuMiA0OS40em01MSAwbDIyLjYgMzkuMiAyMi42LTM5LjJ6IiBmaWxsPSIjYzE2MGVmIi8+CiAgPHBhdGggZD0iTTUyLjYgNUwzMCA0NC4yaDQ1LjJ6TTI3LjIgNDkuNGwyMi43IDM5LjEgMjIuNi0zOS4xem01MSAwTDU1LjYgODguNWg0NS4yeiIgZmlsbD0iI2E2MGVlNSIvPgo8L3N2Zz4=&labelColor=374151&color=60a5fa
  [open-vsx-url]: https://open-vsx.org/extension/blazing-fast-rustjs/blazing-fast-rust

  <!-- Insert new entries lexicographically by language code.
     For example given below is the same order as these files appear on page:
     https://github.com/manfromexistence/blazing-fast-rust/tree/main/packages/@blazing-fast-rust/blazing-fast-rust -->

  [हिन्दी](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.hi.md) | [English](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.md) | [Français](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.fr.md) | [繁體中文](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.zh-TW.md) | [简体中文](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.zh-CN.md) | [日本語](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.ja.md) | Português do Brasil | [한국어](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.kr.md) | [Русский](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.ru.md) | [Українська](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.uk.md)
</div>

<br>

**blazing-fast-rust** é um conjunto de ferramentas de alto desempenho para projetos web, visando fornecer recursos de desenvolvimento para manter a saúde desses projetos.

**blazing-fast-rust é um [formatador rápido](./benchmark#formatting)** para _JavaScript_, _TypeScript_, _JSX_, e _JSON_ que atinge **[97% de compatibilidade com o _Prettier_](https://console.algora.io/challenges/prettier)**.

**blazing-fast-rust é um [linter eficiente](https://github.com/manfromexistence/blazing-fast-rust/tree/main/benchmark#linting)** para _JavaScript_, _TypeScript_, e _JSX_ que possui **[mais de 270 regras](https://manfromexistence.vercel.app/linter/rules/)** do ESLint, typescript-eslint, e de [outras fontes](https://github.com/manfromexistence/blazing-fast-rust/discussions/3).
Ele **fornece diagnósticos detalhados e contextualizados** que ajudam você a melhorar seu código e se tornar um programador melhor!

**blazing-fast-rust** é projetado desde o início para ser usado [interativamente dentro de um editor](https://manfromexistence.vercel.app/guides/editors/first-party-extensions/).
Isso permite formatar e lintar códigos malformados enquanto você programa.

### Instalação

```shell
npm install --save-dev --save-exact @blazing-fast-rust/blazing-fast-rust
```

### Uso

```shell
# formatar arquivos
npx @blazing-fast-rust/blazing-fast-rust format --write ./src

# lintar arquivos
npx @blazing-fast-rust/blazing-fast-rust lint ./src

# executar formatação, lint, etc. e aplicar as sugestões seguras
npx @blazing-fast-rust/blazing-fast-rust check --write ./src

# verificar todos os arquivos contra formatação, lint, etc. em ambientes CI
npx @blazing-fast-rust/blazing-fast-rust ci ./src
```

Se você quiser experimentar o blazing-fast-rust sem instalá-lo, use o [playground online](https://manfromexistence.vercel.app/playground/), compilado para WebAssembly.

## Documentação

Confira nossa [página inicial][blazing-fast-rustjs] para saber mais sobre o blazing-fast-rust,
ou vá ao [Guia de Introdução][getting-started] para começar a usar o blazing-fast-rust.

## Mais sobre o blazing-fast-rust

**blazing-fast-rust** tem padrões robustos e não requer configuração.

**blazing-fast-rust** visa suportar [todas as principais linguagens][language-support] do desenvolvimento web moderno.

**blazing-fast-rust** [não requer Node.js](https://manfromexistence.vercel.app/guides/manual-installation/) para funcionar.

**blazing-fast-rust** tem suporte de primeira linha para LSP, com um

 parser sofisticado que representa o texto-fonte em sua total fidelidade e recuperação de erro de ponta.

**blazing-fast-rust** unifica funcionalidades que anteriormente eram ferramentas separadas. Construindo sobre uma base compartilhada, podemos fornecer uma experiência coesa para processar código, exibir erros, paralelizar trabalho, cache e configuração.

Leia mais sobre nossa [filosofia de projeto][blazing-fast-rust-philosophy].

**blazing-fast-rust** é licenciado sob [MIT](https://github.com/manfromexistence/blazing-fast-rust/tree/main/LICENSE-MIT) ou [Apache 2.0](https://github.com/manfromexistence/blazing-fast-rust/tree/main/LICENSE-APACHE) e moderado sob o [Código de Conduta do Contribuidor](https://github.com/manfromexistence/blazing-fast-rust/tree/main/CODE_OF_CONDUCT.md).

## Patrocinadores

### Patrocinadores Ouro

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


### Patrocinadores Prata

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

### Patrocinadores Bronze

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

[blazing-fast-rustjs]: https://manfromexistence.vercel.app/pt-br/
[blazing-fast-rust-philosophy]: https://manfromexistence.vercel.app/pt-br/internals/philosophy/
[language-support]: https://manfromexistence.vercel.app/pt-br/internals/language-support/
[getting-started]: https://manfromexistence.vercel.app/pt-br/guides/getting-started/

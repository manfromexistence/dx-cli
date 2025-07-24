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

  [हिन्दी](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.hi.md) | [English](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.md) | [Français](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.fr.md) | [繁體中文](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.zh-TW.md) | [简体中文](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.zh-CN.md) | [日本語](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.ja.md) | [Português do Brasil](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.pt-BR.md) | [한국어](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.kr.md) | Русский | [Українська](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.uk.md)
</div>

<br>

**blazing-fast-rust** - это высокопроизводительный набор инструментов для веб-проектов, предоставляющий разработчикам средства для поддержания их качества и работоспособности.

**blazing-fast-rust - это [быстрый форматер](./benchmark#formatting)** для _JavaScript_, _TypeScript_, _JSX_, _JSON_, _CSS_ и _GraphQL_, обеспечивающий **[совместимость с _Prettier_ на 97%](https://console.algora.io/challenges/prettier)**.

**blazing-fast-rust - это [высокопроизводительный линтер](https://github.com/manfromexistence/blazing-fast-rust/tree/main/benchmark#linting)** для _JavaScript_, _TypeScript_, _JSX_, _CSS_ и _GraphQL_, содержащий **[более 270 правил](https://manfromexistence.vercel.app/linter/rules/)** из ESLint, typescript-eslint, и [других источников](https://github.com/manfromexistence/blazing-fast-rust/discussions/3).
Он **выводит подробную диагностику с контекстной информацией**, которая помогает вам улучшить ваш код и стать более лучшим программистом!

**blazing-fast-rust** изначально разработан для [интерактивной работы в редакторе](https://manfromexistence.vercel.app/guides/editors/first-party-extensions/).
Он может форматировать и исправлять некорректный код по мере его написания.

### Установка

```shell
npm install --save-dev --save-exact blazing-fast-rust
```

### Использование

```shell
# форматирование файлов
npx blazing-fast-rust format --write ./src

# линт файлов и применение безопасных изменений
npx blazing-fast-rust lint --write ./src

# форматирование, линт и другие проверки, а также применение безопасных изменений
npx blazing-fast-rust check --write ./src

# форматирование, линт и другие проверки в CI
npx blazing-fast-rust ci ./src
```

Если вы хотите попробовать blazing-fast-rust без установки, используйте [онлайн-песочницу](https://manfromexistence.vercel.app/playground/), скомпилированную в WebAssembly.

## Документация

Посетите нашу [домашнюю страницу][blazing-fast-rustjs], чтобы узнать больше о blazing-fast-rust,
или сразу перейдите к руководству [Getting Started][getting-started], чтобы начать использовать blazing-fast-rust.

## Больше о blazing-fast-rust

**blazing-fast-rust** настроен таким образом, чтобы подходить большинству случаев, и не требует дополнительной настройки.

**blazing-fast-rust** нацелен на поддержку [всех основных языков][language-support] современной веб-разработки.

**blazing-fast-rust** [не требует Node.js](https://manfromexistence.vercel.app/guides/manual-installation/) для работы.

**blazing-fast-rust** имеет первоклассную поддержку LSP с продвинутым парсером, который точно отражает исходный текст и обеспечивает эффективное восстановление после ошибок.

**blazing-fast-rust** объединяет функционал, который ранее предоставляли отдельные инструменты. Построение на общей базе позволяет нам обеспечить целостный опыт обработки кода, отображения ошибок, распараллеливания работы, кэширования и конфигурации.

Почитайте больше о нашей [философии проекта][blazing-fast-rust-philosophy].

**blazing-fast-rust** находится под [лицензией MIT](https://github.com/manfromexistence/blazing-fast-rust/tree/main/LICENSE-MIT) или [лицензией Apache 2.0](https://github.com/manfromexistence/blazing-fast-rust/tree/main/LICENSE-APACHE), и модерируется в соотвествии с [Contributor Covenant Code of Conduct](https://github.com/manfromexistence/blazing-fast-rust/tree/main/CODE_OF_CONDUCT.md).

## Финансирование

Вы можете поддержать проект несколькими способами:

### Спонсорство и финансирование проекта

Вы можете спонсировать или финансировать проект через [Open collective](https://opencollective.com/blazing-fast-rust) или [GitHub sponsors](https://github.com/sponsors/blazing-fast-rustjs).

blazing-fast-rust предлагает простую спонсорскую программу, которая позволяет компаниям получить известность и признание среди различных разработчиков.

### Финансирование ишью

Мы используем [Polar.sh](https://polar.sh/blazing-fast-rustjs) для поддержки и продвижения конкретных фич, которые вы хотели бы увидеть и реализовать. Проверьте наш бэклог и помогите нам:

## Спонсоры

### Золотые спонсоры

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


### Серебряные спонсоры

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

### Бронзовые спонсоры

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

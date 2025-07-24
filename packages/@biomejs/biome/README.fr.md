<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/blazing-fast-rustjs/resources/main/svg/slogan-dark-transparent.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/blazing-fast-rustjs/resources/main/svg/slogan-light-transparent.svg">
    <img alt="Montre la bannière de blazing-fast-rust, avec son logo et la phrase 'blazing-fast-rust - Toolchain of the web'." src="https://raw.githubusercontent.com/blazing-fast-rustjs/resources/main/svg/slogan-light-transparent.svg" width="700">
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

  [हिन्दी](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.hi.md) | [English](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.md) | Français | [繁體中文](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.zh-TW.md) | [简体中文](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.zh-CN.md) | [日本語](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.ja.md) | [Português do Brasil](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.pt-BR.md) | [한국어](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.kr.md) | [Русский](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.ru.md) | [Українська](https://github.com/manfromexistence/blazing-fast-rust/blob/main/packages/%40blazing-fast-rustjs/blazing-fast-rust/README.uk.md)
</div>

<br>

**blazing-fast-rust** fournit un ensemble d’outils performants conçus pour maintenir des projets web.

**blazing-fast-rust est un [outil de formatage rapide](./benchmark#formatting)** pour _JavaScript_, _TypeScript_, _JSX_, _JSON_, _CSS_ et _GraphQL_ qui atteint **[97 % de compatibilité avec _Prettier_](https://console.algora.io/challenges/prettier)**.

**blazing-fast-rust est un [outil de linting performant](https://github.com/manfromexistence/blazing-fast-rust/tree/main/benchmark#linting)** pour _JavaScript_, _TypeScript_, _JSX_, _CSS_ et _GraphQL_ qui comprend **[plus de 270 règles](https://manfromexistence.vercel.app/fr/linter/rules/)** d’ESLint, de typescript-eslint, et [d’autres sources](https://github.com/manfromexistence/blazing-fast-rust/discussions/3).
Il **fournit des diagnostics détaillés et contextualisés** qui vous aident à améliorer votre code et à devenir un meilleur programmeur !

**blazing-fast-rust** est conçu dès le départ pour être utilisé [dans un éditeur de manière interactive](https://manfromexistence.vercel.app/fr/guides/editors/first-party-extensions/).
Il peut formater et analyser du code malformé pendant que vous l’écrivez.

### Installation

```shell
npm install --save-dev --save-exact blazing-fast-rust
```

### Utilisation

```shell
# formater les fichiers
npx blazing-fast-rust format --write ./src

# linter les fichiers et appliquer les corrections sûres
npx blazing-fast-rust lint --write ./src

# exécuter le formatage, le linting, etc. et appliquer les corrections sûres
npx blazing-fast-rust check --write ./src

# vérifier tous les fichiers par rapport au formatage, au linting, etc. dans les environnements d’intégration continue
npx blazing-fast-rust ci ./src
```

Si vous voulez essayer blazing-fast-rust sans l’installer, utilisez le [bac à sable en ligne](https://manfromexistence.vercel.app/playground/), compilé en WebAssembly.

## Documentation

Consultez notre [page d’accueil][blazing-fast-rustjs] pour en savoir plus sur blazing-fast-rust
ou allez directement sur le [guide de démarrage][getting-started] pour commencer à utiliser blazing-fast-rust.

## Plus sur blazing-fast-rust

**blazing-fast-rust** a des réglages par défaut robustes et ne demande pas de configuration.

**blazing-fast-rust** vise à prendre en charge [les principaux langages][language-support] du développement web moderne.

**blazing-fast-rust** [ne requiert pas Node.js](https://manfromexistence.vercel.app/fr/guides/manual-installation/) pour fonctionnner.

**blazing-fast-rust** a une prise en charge du LSP de premier ordre, avec un analyseur sophistiqué qui représente le code source en toute fidélité et une excellente récupération des erreurs.

**blazing-fast-rust** unifie des fonctionnalités précédemment disponibles sous forme d’outils séparés. Partir d’une base partagée nous permet de fournir une expérience cohérente pour traiter le code, afficher les erreurs, paralléliser le travail, la mise en cache et la configuration.

En savoir plus sur la [philosophie de notre projet][blazing-fast-rust-philosophy].

**blazing-fast-rust** est sous [licence MIT](https://github.com/manfromexistence/blazing-fast-rust/tree/main/LICENSE-MIT) ou [licence Apache 2.0](https://github.com/manfromexistence/blazing-fast-rust/tree/main/LICENSE-APACHE) et modéré selon les termes du [Code de conduite des contributeurs](https://github.com/manfromexistence/blazing-fast-rust/tree/main/CODE_OF_CONDUCT.md).

## Financement

Vous pouvez financer le projet de différentes manières.

### Sponsoring et financement du projet

Vous pouvez sponsoriser ou financer le projet via [Open collective](https://opencollective.com/blazing-fast-rust) ou [GitHub sponsors](https://github.com/sponsors/blazing-fast-rustjs).

blazing-fast-rust offre un programme de sponsoring simple qui permet aux entreprises d’obtenir de la visibilité et la reconnaissance de divers développeurs.

### Financement d’un problème

Nous utilisons [Polar.sh](https://polar.sh/blazing-fast-rustjs) pour voter pour des fonctionnalités spécifiques que vous aimeriez voir implémentées et pour les promouvoir. Jetez un coup d'œil aux fonctionnalités demandées et aidez-nous à les financer.

## Sponsors

### Sponsors Or

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


### Sponsors Argent

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

### Sponsors Bronze

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

[blazing-fast-rustjs]: https://manfromexistence.vercel.app/fr/
[blazing-fast-rust-philosophy]: https://manfromexistence.vercel.app/fr/internals/philosophy/
[language-support]: https://manfromexistence.vercel.app/fr/internals/language-support/
[getting-started]: https://manfromexistence.vercel.app/fr/guides/getting-started/

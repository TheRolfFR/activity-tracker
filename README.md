<h1 align="center">
<br>
<img src="./Images/icon.png" alt="logo" width="128" />
<br>
Activity Tracker
</h1>

<h4 align="center">Custom work activity tracker with <a href="https://tauri.app/" target="_blank">Tauri</a>, <a href="https://www.rust-lang.org/" target="_blank">Rust</a> and <a href="https://kit.svelte.dev/" target="_blank">SvelteKit</a></h4>
<br>

<div align="center">
    <img src="./Images/activity-tracker.png" />
</div>

## Technologies

Front-end: Svelte and <a href="https://fluent-svelte.vercel.app/" href="_blank" >Fluent Svelte</a> for visual components, <a href="https://www.npmjs.com/package/svelte-tiny-linked-charts" target="_blank">svelte-tiny-linked-charts</a> for charts visualization.

Back-end: TypeScript bindings for Svelte are provided by the <a href="https://crates.io/crates/ts-rs" target="_blank">ts-rs</a> crate. 
serde and serde_json for serialization. <a href="https://crates.io/crates/rdev" targret="_blank">rdev</a> for global input events. 
Massive use of <a href="https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html">MPSC channels</a> in Rust code because data goes only one way.

## How to develop

Recommended IDE Setup:

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

Prepare your environment: follow the guide from https://tauri.app/v1/guides/getting-started/prerequisites.

Install JS and Rust dependencies

```
pnpm install && cd src-tauri && cargo build
```

Start dev command from root project

```
cargo tauri dev
```
depends on the package manager used to install tauri CLI.

# Nautile Navigateur

Nautile Navigateur est un navigateur Web écrit from scratch en Rust. La base actuelle pose un workspace Cargo multi-crates, un shell desktop `winit`/`wgpu`, un mode headless, et des frontières stables pour HTML, DOM, CSS, style, layout, paint, compositor, JavaScript, Web APIs, stockage, sécurité, IPC, DevTools et tests.

## Commandes

```bash
cargo check --workspace
cargo fmt --all
cargo run -p nautile-headless -- --url about:version --dump
cargo run -p nautile-desktop -- about:blank
```

## Architecture courte

Le pipeline cible est : URL → Browser Core → Navigation → Network/Loader → HTML → DOM → CSS → Style → Layout → Paint → Compositor → GPU. Le pipeline dynamique est : JS → DOM bindings → mutations → invalidation style/layout/paint → frame scheduler. Les inputs suivent : `winit` → platform event → routing navigateur → hit test → DOM event → JS listener.

## Roadmap courte

V0 établit le workspace, la fenêtre, l'event loop et les pages `about:`. V1-V5 ajoutent URL/HTTP/HTML/DOM/CSS/Layout/Paint/Compositor. V6-V9 ajoutent JavaScript, bindings DOM, events, forms, fetch, cookies et storage. V10-V14 renforcent layout moderne, multi-process, DevTools, WPT/Test262 et performance.

# Architecture de Nautile Navigateur

Nautile vise un navigateur Web complet écrit en Rust, sans WebView ni moteur externe. Le code est découpé en crates `nautile_*` pour isoler les responsabilités : shell, browser core, navigation, réseau, loaders, HTML, DOM, CSS, style, layout, paint, compositor, GPU, JavaScript, Web APIs, event loop, stockage, sécurité, IPC/process et DevTools.

## Pipeline navigation/rendu

URL saisie → Browser UI → Browser Core → NavigationController → NetworkService/DocumentLoader → HTML tokenizer/tree builder → DOM tree → CSS parser → selector matching/cascade → computed style tree → layout tree → fragment tree → display list → paint chunks → layer tree → compositor frame → `wgpu` surface.

## Pipeline JavaScript dynamique

Script/event handler → JS lexer/parser → AST → bytecode compiler → VM/runtime → DOM bindings → mutation DOM → invalidation style/layout/paint → frame scheduler → compositor.

## Pipeline input

`winit` event → événement plateforme → routage Browser Core → hit testing layout/compositor → focus manager → DOM event target → capture/target/bubble → JS listeners → mutation éventuelle → rendering update.

## Process model

V0 fonctionne en single-process avec des frontières IPC déjà nommées. La cible sépare browser, renderer par site/origin, network, GPU, storage, utility et DevTools processes avec messages sérialisables et crash recovery.

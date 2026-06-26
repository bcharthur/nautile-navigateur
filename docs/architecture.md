# Architecture Nautile

## Vue d'ensemble

Nautile est un navigateur web complet, développé en Rust, organisé en workspace multi-crates.

## Processus

```
browser process
├── network service process
├── gpu process
├── storage process
├── renderer process (1 par site)
└── utility process
```

## Moteurs

| Moteur | Crates | Rôle |
|--------|--------|------|
| Réseau | net_* | HTTP, TLS, DNS, cache, cookies |
| HTML | html_* | Tokenizer, tree builder |
| DOM | dom_* | Document, nodes, events, shadow DOM |
| CSS | css_* + style_engine | Parse, cascade, computed styles |
| Layout | layout_* | Block, flex, grid, inline, text |
| Paint | paint | Display list, stacking contexts |
| Compositor | compositor | Layers, raster, GPU frames |
| JavaScript | js_* | Lexer → parser → AST → bytecode → VM → GC |
| Bindings | bindings_* | Pont JS <-> DOM/Web APIs |
| Web APIs | webapi_* | console, timers, fetch, workers… |
| Stockage | storage_* | IndexedDB, localStorage, cookies, cache |
| Sécurité | security_* | Origin, CSP, CORS, sandbox |
| IPC | ipc_core | Messages inter-processus |
| DevTools | devtools_* | Inspection DOM, réseau, console |

## Pipeline de rendu

```
URL
→ réseau (net_fetch)
→ HTML parser (html_parser)
→ DOM (dom_core)
→ CSS parser (css_syntax + css_cascade)
→ Style engine (style_engine)
→ Layout (layout_core + layout_block/flex/grid/inline)
→ Paint (paint)
→ Compositor (compositor)
→ GPU (gpu_backend)
→ Écran
```

## Event loop

```
1. Tâche (task_queue)
2. Microtâches (promises, MutationObserver)
3. Recalcul style si dirty
4. Layout si dirty
5. Paint si dirty
6. requestAnimationFrame callbacks
7. Composite + présentation frame
```

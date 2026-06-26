# Roadmap Nautile

## Version 0 — Infrastructure ✅ (en cours)
- [x] Cargo workspace multi-crates
- [x] Crates fondatrices (common, dom_core, js_ast, js_vm, js_gc)
- [x] Event loop de base
- [x] IPC messages
- [x] App headless + desktop stubs
- [ ] Logging structuré
- [ ] Window vide (platform crate)
- [ ] Test runner headless

## Version 1 — HTML + DOM vivant
- [ ] URL parser conforme WHATWG
- [ ] HTTP GET simple (TCP + TLS basique)
- [ ] HTML tokenizer complet
- [ ] HTML tree builder (avec gestion des erreurs)
- [ ] DOM arena + traversal
- [ ] about:blank, about:version
- [ ] DOM dump headless

## Version 2 — CSS + Premier rendu
- [ ] CSS tokenizer/parser
- [ ] Sélecteurs simples (tag, class, id)
- [ ] Cascade + héritage
- [ ] Computed styles
- [ ] Block layout
- [ ] Inline + text basique
- [ ] Paint rectangles + texte
- [ ] Fenêtre desktop (winit)

## Version 3 — JS moteur initial
- [ ] JS lexer
- [ ] JS parser → AST
- [ ] Interpréteur AST
- [ ] Valeurs JS (Undefined, Null, Boolean, Number, String, Object)
- [ ] Fonctions + closures
- [ ] DOM bindings basiques (document.querySelector, textContent)
- [ ] Événements click

## Version 4 — Event Loop Web
- [ ] Task queue complète
- [ ] Microtask queue (Promises)
- [ ] setTimeout / setInterval
- [ ] requestAnimationFrame
- [ ] DOM mutation → style/layout dirty
- [ ] Repaint incrémental

## Version 5 — Layout sérieux
- [ ] Inline layout correct + line breaking
- [ ] Fonts (fontdue ou maison)
- [ ] Images (PNG/JPEG/WebP)
- [ ] Position absolute/fixed/sticky
- [ ] Overflow + scroll
- [ ] Flexbox
- [ ] Z-index + stacking contexts

## Version 6 — JS VM bytecode
- [ ] Bytecode compiler
- [ ] Bytecode VM (registres ou pile)
- [ ] Call stack + exceptions
- [ ] Prototypes + classes
- [ ] Modules ESM
- [ ] async/await
- [ ] Premiers tests Test262

## Version 7 — Réseau Web moderne
- [ ] TLS 1.3 (rustls temporaire puis maison)
- [ ] HTTP/1.1 complet
- [ ] HTTP/2
- [ ] Cookies
- [ ] Cache HTTP
- [ ] Fetch API
- [ ] WebSocket
- [ ] CORS

## Version 8 — Multi-processus
- [ ] Browser process / Renderer process
- [ ] Network process
- [ ] GPU process
- [ ] IPC via Unix socket / shared memory
- [ ] Crash recovery renderer
- [ ] SiteInstance / Origin isolation

## Version 9 — Web APIs modernes
- [ ] Canvas 2D
- [ ] Web Workers
- [ ] Service Workers
- [ ] IndexedDB
- [ ] Cache API
- [ ] Performance API
- [ ] WebAssembly

## Version 10 — Performance
- [ ] Style invalidation fine
- [ ] Layout incrémental
- [ ] Paint invalidation / damage tracking
- [ ] Compositor-thread scrolling
- [ ] Raster cache
- [ ] JS baseline JIT
- [ ] GC générationnel

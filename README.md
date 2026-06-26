# Nautile

Navigateur web souverain francais - moteur embarque no_std concu pour Bouchaud OS.

## Architecture

- browser/mod.rs    - BrowserState, API publique
- browser/state.rs  - onglets, historique, session JS
- browser/pages.rs  - pages internes (about:bouchaud, about:calc, about:wasm)
- browser/loader.rs - pipeline reseau (HTTP/HTTPS souverain, file:)
- browser/engine/   - pont vers gui::engine (web, js, image)
- browser/ui/chrome.rs - chrome complet (onglets, barre d'adresse, evenements)
- browser/ui/theme.rs  - palette Chrome/Firefox-inspired

## Integration Bouchaud OS

Dependances OS : crate::net (TLS 1.3), crate::drivers::gfx (framebuffer VBE), linked_list_allocator

## Licence

MIT

//! Nautile Navigateur — module source integre dans Bouchaud OS.
//!
//! Ce repertoire `src/` contient les sources du navigateur qui sont
//! copiees directement dans `bouchaud-os/src/browser/` et compilees
//! dans le contexte du noyau (`no_std`).
//!
//! Architecture du module navigateur :
//!
//!  ┌──────────────┬────────────────────────────────────────────┐
//!  │ state        │ Onglets, historique de navigation, JS      │
//!  │ loader       │ Pipeline reseau → HTML → Page rendue       │
//!  │ pages        │ Pages internes : about:bouchaud, calc...   │
//!  │ engine/      │ Pont vers le moteur de rendu web           │
//!  │ ui/theme     │ Palette visuelle (couleurs, dimensions)    │
//!  │ ui/chrome    │ Chrome du navigateur (onglets, barre URL)  │
//!  └──────────────┴────────────────────────────────────────────┘
//!
//! Pour les crates modulaires (architecture future), voir `crates/`.

pub mod browser;

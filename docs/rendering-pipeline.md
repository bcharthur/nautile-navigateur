# Rendering pipeline

DOM tree → Style tree → Layout tree → Fragment tree → Display list → Paint chunks → Layer tree → Compositor frame → `wgpu` surface.

Chaque étape produit une structure de données explicite et testable. Le renderer doit rester utilisable en headless pour dumps et tests de conformité.

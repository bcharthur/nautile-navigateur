# JavaScript engine

Le moteur JavaScript de Nautile sera interne : lexer, parser, AST, bytecode, VM, runtime, objets, GC, modules, regex, bindings DOM et Web APIs.

Les promises s'intègrent à la microtask queue du Web event loop. Les DOM bindings exposent Window, Document, Node, Element, EventTarget, Console, timers et fetch. Un JIT baseline est réservé à une version future après stabilisation de la VM et du GC.

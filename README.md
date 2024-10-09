# hyperULE

hyperULE is a small and simple language with basic features and C-like syntax.

The main goals are as following:

- Continue to learn the inner workings of a compiler, especially transpiling into a shitty, less powerful language.
- Learning basic compiler optimizations
- Continue learning Rust 🦀
- *Bring DX features to to ULE based HHR:*
  - Functions
  - Arrays
  - Structs
  - Type-Safety
- *Work around common but hard-to-debug issues in ULE:*
  - `StrToInt(derp + """)`
  - other... (tbd)
- *If successful:*
  - *Create backend for Zebra, Newland/Lua and Cognex?*<br/>
    *--> One code for every platform*


## Progress

- [x] ~~Tokenizer~~
- [ ] **AST-Parser (Current Task)**
- [ ] Static Analyzer
- [ ] Code Generator
- [ ] Optimizer
- [ ] Extendable Backend
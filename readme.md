# Kyle's Unnamed Language Project

roadmap:
1. ~~Type checking~~
2. ~~Finish variables~~
  - ~~Mutability~~
  - ~~Type checking~~
3. Lists!
  - List creation
    - range
    - empty list
  - List type
  - Typed empty list
4. Const lists / tuples
  - Creation
  - tuple type
  - destructuring

todos:

- [x] variables
  - [x] create
  - [x] mutate syntax
  - [x] const
- [ ] loop expressions
  - [ ] `while`
  - [ ] `for` (iterator)
  - [ ] infinite `loop`
- [ ] match expressions
  - [ ] match on type
  - [ ] match on value
  - [ ] match on condition
  - [ ] check "completeness" of branches
  - [ ] destructuring / capturing
    - [ ] enum
    - [ ] struct
    - [ ] list
      - [ ] head / tail
      - [ ] positional destructuring
- [ ] block `return`
- [ ] exceptions
- [ ] function application / pipe with auto curry
- [ ] data structures
  - [ ] list
  - [ ] tuple (aka. const list)
  - [ ] struct
  - [ ] enum
  - [ ] destructuring
- [ ] interfaces
  - [ ] definition
  - [ ] implementation
  - [ ] method calling
  - [ ] operator implementing (i.e. struct + struct)
- [ ] type system
  - [x] utility types
    - [x] or (`|`)
    - [x] function (`(type) => type`)
    - [x] number
    - [x] string
    - [x] boolean
  - [ ] type definitions + binding
    - [x] function argument types
    - [x] function return types
    - [x] inferred return types
    - [ ] let variable type
    - [ ] inferred recursive return types
  - [ ] type checking
    - [x] function argument types
    - [x] operator argument types
    - [ ] `let` variable creation
    - [ ] variable mutation
  - [ ] type as argument
  - [ ] type inference
- [ ] imports & exports
- [ ] I/O
  - [ ] std
  - [ ] file
  - [ ] network
- [ ] concurrency
  - [ ] async / await (implicit virtual threading)
  - [ ] multi-threading (explicit)
  - [ ] mutex
- [ ] compile (llvm? / wasm? / bytecode?)
- standard library
  - [ ] utility functions
    - [x] print
    - [ ] input
  - [ ] enums
    - [ ] `Optional`
    - [ ] `Result`

housekeeping:

- [ ] testing
  - [ ] unit tests
    - [ ] tokeniser
    - [ ] parser
    - [ ] subtype typechecking
    - [ ] program typechecking
    - [ ] type inference
    - [ ] interpreting
  - [ ] integration tests
    - [ ] recursion
      - [ ] type inference
      - mutual recursion
    - [ ] variables
      - [ ] mutability
      - [ ] type checking
- [ ] examples
- [ ] documentation
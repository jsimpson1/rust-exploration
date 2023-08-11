# rust-exploration
Rust language exploration project

## Language exploration
* Bruno saliencies for languages (https://pad.cj.dev/p/mentoring-07-21-2023)
  * Important items about a programming language:
    * IO Operations - Filesystem, Streams, Network Operations
    * Syntax
    * Loops, Variables and Functions, building system/depedency manager
    * Macro? Any special tools designed in that language?
    * How Community support/adoption?
    * What kinda problem I think this language can help me to solve
    * Unit tests system

### Rust
* Resources from Bruno
    * code exercises
        * https://adventofcode.com/2022
    * references
        * https://doc.rust-lang.org/book/
    * command line
        * check version
            * compiler
              * rustc
                * `rustc --version`
                * compile
                  * `rustc main.rs`
                * run
                  * `./main`
        * update rust
            * `rustup update`
        * package manager aka `cargo`
          * `cargo --version`
          * initialize project
            * `cargo init .`
          * `cargo build`
              * optimized for rebuilding
              * compiles and generates executables
              * executables are in `target/debug`
          * `cargo build --release`
              * optimized for run speed
              * executables are in `target/release`
          * `cargo run`
              * complies and runs
          * `cargo check`
              * compiles and does not generates executables
        * crates
            * https://crates.io/crates/rand
    * intellij
        * you need to refresh cargo when pulling new crates / modules
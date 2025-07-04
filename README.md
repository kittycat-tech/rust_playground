# rust_playground
Playing rust for fun!

My Rust learning journal: https://github.com/kittycat-tech?tab=projects

RustBook source: https://www.scs.stanford.edu/~zyedidia/docs/rust/rust_book.pdf

Commands:
- cargo init (to initiate files with gitignore and lock)
- cargo build (to build and also download packages in toml file)
- cargo run (to run the project)
- cargo clean (to clean from caches because my code was correct but still failed to build, due to uncleaned previous wrong codes. Can't believe this thing is real.)

You need to click save before re-running cargo or else, you continue to get your previous result

Installations:
installed git firefly for gitignore
installed extension for .toml

- TOML (Tom’s Obvious, Minimal Language) format, which is Cargo’s
configuration format.

- The & indicates that this argument is a reference, which gives you a way to let multiple parts
of your code access one piece of data without needing to copy that data into memory
multiple times.


Cargo understands Semantic Versioning (sometimes called SemVer), which is a standard for writing version numbers. The specifier 0.8.5 is actually shorthand for ^0.8.5 , which means any version that is at least 0.8.5 but below 0.9.0.

- note: The next time you run cargo build , Cargo will update the registry of crates available and reevaluate your rand requirements according to the new version you have specified.

My First day play: https://github.com/kittycat-tech/rust_playground/blob/main/secret_number_game/src/main.rs

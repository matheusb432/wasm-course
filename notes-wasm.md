# WebAssembly

- WebAssembly (WASM) is a binary instruction format for a stack-based virtual machine designed to be a compilation target for high-level languages like Rust/C++/C#/Go
  - Devs tend to prefer a language with a strong background in systems programming, like Rust, to write code that compiles to WASM.
- WASM enables execution of code at near-native speed in the browser.
- WASM is most useful for CPU-intensive tasks like image processing, physics simulations, and cryptography.

## Performance

- WASM can be faster than JS because it's a lower-level language that's closer to the hardware.
- However, there is currently no way to directly call WASM from JS. Instead, you have to use a JS API to load and run WASM code.
  - This incurs a performance penalty because of the overhead of calling between JS and WASM.

## Use Cases

- WASM is useful for CPU-intensive tasks like image processing, physics simulations, and cryptography.
- WASM can also be used to build entire apps, and some GUI Rust frameworks like [Dioxus](https://dioxuslabs.com/) and [Leptos](https://leptos.dev/) use it and are much faster than most JS frameworks, even with the performance penalty of calling between JS and WASM.
  - Currently (July 2023), Dioxus & Leptos are faster than React/Vue/Angular, and are roughly as fast as SolidJS (Source: [JS Framework Benchmarks](https://krausest.github.io/js-framework-benchmark/2023/table_chrome_114.0.5735.90.html))

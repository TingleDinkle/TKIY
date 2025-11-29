# TKIY Code Editor: Development Post-Mortem

This document details the technical challenges encountered during the conversion of "The Yellow Compiler" from a Rust CLI tool to a WebAssembly-powered React 3D application, and the specific solutions implemented to resolve them.

## 1. The "Runtime Horror" (Recursive Use Error)

**Problem:** 
After running the interpreter once, subsequent attempts to run code resulted in a JavaScript error: 
`Error: recursive use of an object detected which would lead to unsafe aliasing in rust`.

**Root Cause:** 
This is a specific behavior of `wasm-bindgen`. When a Rust method is called from JS, it locks the underlying Rust struct. If the Rust code panics (crashes) without unwinding the stack cleanly, the "lock" is never released. When JS tries to call the method again, it sees the lock is still held and throws this error to prevent memory corruption.

**The Panic Source:**
The underlying panic was identified as `assertion failed: psize >= size + min_overhead` in `dlmalloc` (the Rust memory allocator for WASM). This indicated heap corruption or a stack overflow smashing into the heap.

**Solution:**
1.  **Pass-by-Reference:** The interpreter was originally passing the Abstract Syntax Tree (AST)—a complex recursive vector structure—**by value** to recursive functions. This triggered massive `memcpy` operations on the stack/heap boundary. We refactored the entire execution pipeline (`execute`, `eval_expr`) to pass data by reference (`&Vec<Stmt>`), significantly reducing memory pressure.
2.  **Panic Removal:** We replaced all instances of `unreachable!()` and `panic!()` in the parser with `Result::Err`. This ensures that invalid tokens return a graceful error string instead of trapping the WASM module.
3.  **Safe RNG:** We removed dependency on `getrandom` (which calls JS crypto) inside the execution loop, replacing it with a pure Rust Xorshift RNG. This ensures execution is "atomic" and cannot trigger JS re-entrancy issues.

## 2. Allocator Corruption (dlmalloc assertion)

**Problem:**
`panicked at /rust/deps/dlmalloc-0.2.10/src/dlmalloc.rs:1201:9`

**Root Cause:**
Deep recursion in the interpreter combined with moving large data structures (the AST) likely corrupted the allocator's metadata headers in the linear memory.

**Solution:**
In addition to the "Pass-by-Reference" fix above, we added an explicit **Recursion Depth Limit**. The `execute_stmt` function now tracks depth and returns an error if it exceeds 100, preventing stack overflows before they corrupt memory.

## 3. Time & Entropy in WASM

**Problem:**
The original CLI relied on `std::time::SystemTime`. In `wasm32-unknown-unknown`, there is no OS clock, causing immediate panics upon instantiation.

**Attempted Solution 1:**
Used `js_sys::Date::now()`. While this worked, it required calling back into JavaScript execution context from Rust. If the JS engine was in a specific state (or if garbage collection triggered), this could destabilize the WASM execution.

**Final Solution:**
We moved to a **Deterministic Entropy System**. 
1.  The RNG is seeded *once* using `Date::now()` only during `Interpreter::new()`.
2.  During execution, "Time" is simulated by an `entropy` counter (`u64`) that increments with every statement executed. This keeps the core logic "pure Rust" and deterministic per run.

## 4. Sanity Persistence & NaN

**Problem:**
1.  Sanity would display as `NaN` or `-0.1%`.
2.  Refreshing the page or clicking "Sign" would sometimes result in an instant "Game Over" because the interpreter state persisted in the React component.

**Solution:**
1.  **Clamping:** The `get_sanity()` method was updated to return `0.0` if the internal value is `NaN` or negative.
2.  **Reset Mechanism:** We added a "RESET REALITY" button in the UI. This explicitly re-instantiates `new YellowWebInterpreter()`, clearing the corrupted/insane state and creating a fresh WASM memory slab.

## 5. CSS Infinite Growth

**Problem:**
The `MinecraftEditor` component would stretch vertically, pushing the "SIGN" button off-screen and breaking the 3D overlay effect.

**Root Cause:**
Flexbox children inside a container with `min-height` often default to their content size, leading to runaway expansion when logs accumulate.

**Solution:**
1.  Applied `min-height: 0` to flex children to force them to respect the parent's constraints.
2.  Set explicit `max-height` on the container.
3.  Set `overflow: auto` on the text areas to ensure scrollbars appear internally.

## 6. HashMap vs. BTreeMap

**Problem:**
`std::collections::HashMap` in Rust uses a random hashing seed by default. In WASM, this pulls in `getrandom`, which pulls in JS crypto bindings. This added unnecessary complexity and potential failure points for re-entrancy.

**Solution:**
Replaced all instances of `HashMap` with `BTreeMap`. `BTreeMap` is deterministic and requires no RNG or system calls, making the WASM binary smaller and more stable.

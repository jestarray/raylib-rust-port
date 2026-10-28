# Project guidance

This file applies to the entire repository.

## Project and scope

- This is a Rust port of raylib's implementation, not a wrapper around the native raylib C library. The Cargo package is `raylib-rs`; the library and imports are named `raylib`. It uses Rust edition 2024 and produces `rlib` and `cdylib` artifacts.
- SDL3 is the platform backend. Rendering targets modern OpenGL and OpenGL ES; legacy OpenGL and software rendering are outside the current scope.
- The port is incomplete. `src/rmodels.rs` currently contains a small set of drawing helpers; a feature name in `Cargo.toml` does not by itself mean the corresponding raylib subsystem is fully implemented.
- Read `README.md` for basic usage and `PORTING_RAYLIB.md` for porting background and intentional differences. The latter includes historical stripping instructions and TODOs; check the current Rust implementation before treating those notes as current behavior.

## Code layout and API placement

The implementation modules retain raylib's C-style names and much of its structure. Their corresponding files in `src/safe/` provide thin safe wrappers with `snake_case` names:

| Implementation | Safe wrapper | Public module |
| --- | --- | --- |
| `src/rcore.rs` | `src/safe/core.rs` | `raylib::core` |
| `src/rsdl.rs` | `src/safe/sdl.rs` | `raylib::sdl` |
| `src/rshapes.rs` | `src/safe/shapes.rs` | `raylib::shapes` |
| `src/rtext.rs` | `src/safe/text.rs` | `raylib::text` |
| `src/rtextures.rs` | `src/safe/textures.rs` | `raylib::textures` |

- **Type-oriented safe bindings belong in methods in `src/types.rs`.** When a raw function takes a type defined there as the object it operates on, expose its safe binding on that type. Examples include `Image::load_image_colors`, `Texture::update_texture`, `Font::measure_text_ex`, `Shader::set_shader_value`, camera coordinate conversions, and input-enum query methods. Use associated functions for type-specific constructors such as `Image::gen_image_color`.
- Global operations belong in the corresponding `src/safe/` file. Existing drawing and render-mode functions also accept types such as `Color`, `Font`, and `Camera2D`; follow their established free-function and drawing-trait patterns when extending those APIs.
- `src/safe/` is a filesystem grouping. `src/lib.rs` keeps `safe` private and re-exports `core`, `sdl`, `shapes`, `text`, `textures`, and `handle` at the crate root. Downstream code uses `raylib::textures`, for example.
- `rcore`, `rsdl`, `rshapes`, `rtext`, and `rtextures` are private implementation modules. Preserve that boundary when adding APIs; expose the safe wrapper or type method through the existing public modules.
- `src/safe/handle.rs` contains `RaylibBuilder`, `RaylibHandle`, the `RaylibDraw` trait, and guards that pair drawing/render-mode begin and end calls through `Drop`. Extend the applicable trait or guard when adding an operation to this interface.
- `src/rlgl.rs` is the public low-level rendering layer and still exposes C-style functions, including unsafe operations. It has no corresponding safe wrapper module. `src/rcamera.rs`, `src/math.rs`, `src/rcolors.rs`, `src/rmodels.rs`, and `src/types.rs` are also public modules with their own existing conventions.
- `examples/` demonstrates the public API; `resources/` contains example textures and shaders. Keep examples synchronized with public API changes.

## Implementation conventions

- Keep ported logic recognizable alongside the corresponding raylib function. Preserve C-style names, constants, comments, and local formatting in the implementation files; use idiomatic `snake_case` in safe wrappers and methods.
- Put behavior changes in the appropriate implementation function and keep wrappers thin. Perform Rust-specific argument conversion and validation at the boundary that can enforce it. Avoid copying implementation logic into additional wrappers.
- Follow existing safe signatures: use `&str` for text, slices for buffers, references for out-parameters, and typed enums where available. Derive counts from slices instead of accepting independent lengths that could disagree.
- Path-taking safe APIs generally accept `AsRef<Path>`. Reuse `safe::core::path_to_str` where the implementation requires UTF-8, and follow the surrounding API's failure/fallback behavior for non-UTF-8 paths.
- Preserve the chosen Rust dependencies: `glam` for vectors/matrices, `image` for image loading and processing, and `swash` for font work. The port intentionally differs from raylib's C implementation in these areas.
- Respect existing `rustfmt` skip attributes and lint allowances in the ported modules. Format changed code to match its surroundings and keep unrelated formatting or naming changes out of the diff.
- When changing public types, preserve relevant representation attributes, enum discriminants, aliases, and optional `serde` derives unless the task explicitly changes their contract.

## Safety, state, and resource lifetimes

- Review the raw implementation before declaring a new wrapper safe. Check pointer validity, slice bounds, dimension/count overflow, ownership, and the lifetime of any retained data. A wrapper around an unsafe function needs to uphold its requirements.
- SDL/window/OpenGL operations rely on process-global state and the appropriate thread/context. `RaylibHandle` and several guards deliberately prevent `Send`/`Sync`; preserve those restrictions and resource borrows. Calling an ordinary safe wrapper does not make the underlying global renderer thread-safe.
- `core::state()` and `core::state_snapshot()` are explicitly unsafe. Preserve their documented aliasing and concurrency requirements, and avoid introducing safe references to mutable live global state.
- Drawing and render-mode guards must end their mode on drop. The texture, shader, and VR guards retain resource lifetimes with `PhantomData`; preserve those relationships when changing signatures.
- GPU-backed resources use explicit unload operations. Cloning a texture/font/render-target value does not allocate an independent GPU resource. Keep resources alive while used, avoid duplicate unloads, and unload owned GPU resources before closing their context. The default font texture is shared and has special unload handling.
- `Image` owns a `Vec<u8>`. Validate dimensions, pixel format, and available bytes before reading buffers or passing them to GL. Reject malformed images, undersized texture updates, and out-of-bounds update rectangles before unsafe access; cover these boundaries with regression tests.
- Shader uniform methods use the sealed `ShaderUniformValue` trait and verify the supplied `ShaderUniformDataType`. Preserve the check before exposing data to the raw upload functions.
- Callback contexts must own data SDL retains. The asynchronous file dialog copies borrowed results into owned paths and communicates through a channel; its callback must also clean up when the receiver has been dropped.

## Math compatibility

- `Vector2`, `Vector3`, and `Vector4` alias glam vector types; `Quaternion` currently aliases `Vec4`, and `Matrix` aliases `glam::Mat4`.
- Preserve raylib's column-major, right-handed matrix behavior. Use glam's `camera::rh::proj::opengl` constructors for OpenGL projection depth conventions.
- `Mat4::default()` is identity. Use `Matrix::ZERO` where the port requires C-style zero initialization.
- Raymath's historical matrix multiplication argument order differs from glam's `*`: `rlMatrixMultiply(left, right)` corresponds to `right * left`. Preserve this behavior when translating or simplifying math, and consult `tests/matrix_glam_compat.rs`.

## Build and validation

Run commands from the repository root:

```text
cargo check --all-targets --locked
cargo test --locked
cargo run --example texture_demo
```

- `cargo check --all-targets --locked` checks the library, integration tests, and examples without opening a graphics window. `cargo test --locked` runs unit, integration, and documentation tests. Run the relevant tests for a code change and include example compilation when public APIs change.
- Examples are interactive and generally need a working SDL3/OpenGL environment. Run them from the root so relative `resources/...` paths resolve. Use a relevant example for visual verification of rendering changes, and report if the environment prevents that check.
- Focused integration tests are `cargo test --test type_methods`, `cargo test --test safety_regressions`, and `cargo test --test matrix_glam_compat`. File-dialog callback tests in `tests/support/file_dialog.rs` are included as unit tests by `src/safe/sdl.rs`, so run them through `cargo test --lib`.
- Prefer tests that verify observable behavior, safety boundaries, or raylib compatibility. Keep ordinary automated tests independent of a live graphics context; preserve tests that reject invalid input before GL is called. Tests needing private implementation details belong in crate-internal test modules.
- Features use the uppercase names in `Cargo.toml`, with a few exceptions such as `serde`. Defaults enable `DESKTOP_DEFAULTS`, including desktop OpenGL 3.3. Desktop OpenGL and OpenGL ES are mutually exclusive, so **do not use `--all-features` as a validation command**. Use `--no-default-features` when selecting an ES configuration, and check affected feature combinations explicitly.
- `GRAPHICS_API_OPENGL_43` includes `GRAPHICS_API_OPENGL_33`; `GRAPHICS_API_OPENGL_ES3` includes `GRAPHICS_API_OPENGL_ES2`. Preserve these feature relationships.
- SDL is a native dependency. `build.rs` can download/cache SDL3 development archives for Windows and Android and copy the Windows runtime into the profile output directory. Other platforms need appropriate native SDL provisioning.
- `NO_DL_SDL` disables automatic downloads, `BUILD_SDL_FROM_SOURCE` delegates the build to `sdl3-sys`, and `SDL_NO_LINK` skips linking/provisioning for externally packaged SDL. Disabling linking does not provide a runnable SDL runtime for tests or examples.
- In the final handoff, state what changed, which checks ran, and any checks that could not be completed. Do not claim runtime or visual validation based only on `cargo check`.

### Test failures observed on Windows (2026-10-28)

- `cargo test --locked --no-fail-fast` ran all test targets. Two existing tests in `tests/safety_regressions.rs` failed: `malformed_image_does_not_read_past_its_buffer` because `is_image_valid()` accepted its short buffer, and `missing_font_file_returns_invalid_font_without_panicking` because an unloaded GL function was called. Other unit, integration, and documentation tests passed. Account for these baseline failures when evaluating changes, and update these notes when the underlying issues are fixed.

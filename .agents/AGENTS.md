# StoneAge (RustEngine3D) Project Rules & Conventions

This file defines the project-specific rules, architectural patterns, and practical lessons learned from the entire project history (~280+ archived coding sessions) for AI agents working on this codebase.

---

## 1. Code Style & Naming Conventions

### 1.1 Struct Fields
- **Underscore Prefix (`_field_name`)**: All struct fields (both `pub` and private) **MUST** begin with an underscore prefix.
  ```rust
  pub struct InventorySlotWidget<'a> {
      pub _inventory_widget: *const InventoryWidget<'a>,
      pub _slot_index: usize,
      pub _base_contents_area_size: Vector2<f32>,
  }
  ```
- **Unused Parameters**: Prefix unused parameters or local variables with an underscore (e.g., `_window_size: &Vector2<i32>`).

### 1.2 Constants & No Magic Numbers
- Define named `SCREAMING_SNAKE_CASE` `const` constants in [game_constants.rs](file:///mnt/Workspace/StoneAge/src/game_module/game_constants.rs) or module headers instead of hardcoding numeric values, dimensions, sound asset paths, or texture keys.

### 1.3 Error Handling & Safety
- **No Blind `.unwrap()`**: Use defensive pattern matching (`if let`, `unwrap_or(...)`, bounds checking).
- Avoid `i16::MIN.abs()` overflow panic when handling gamepad axis inputs; use `saturating_abs()` or clamp to `-i16::MAX`.
- **Comments**: Keep comments minimal and in **English**. Do not clutter code with verbose explanations. Preserve existing docstrings.

---

## 2. Architecture & Design Patterns

### 2.1 Service Locator Pattern
- Access all core managers and engine resources globally via service locators. **Do NOT pass manager references as function arguments**.
  - Game layer: `get_game_ui_manager()`, `get_game_ui_manager_mut()`, `get_character_manager()`, `get_character_manager_mut()`, `get_item_manager_mut()`.
  - Engine layer: `get_engine_resources()`, `get_engine_resources_mut()`, `get_audio_manager_mut()`, `get_scene_manager()`.

### 2.2 Resource Loading & Streaming Architecture
1. **Engine Resource Policy (`is_engine_resource`)**:
   - Only fundamental engine resources located in `ENGINE_RESOURCE_PATH` (default fallback materials, `common/default` texture, shaders, render targets) load synchronously at startup and remain pinned.
   - **Never hardcode game asset lists to load synchronously**.
2. **Late-Binding & Dynamic Async Loading**:
   - All character, prop, scene mesh, and texture assets are loaded asynchronously via worker threads.
   - Textures bind placeholder `DEFAULT_TEXTURE_NAME` initially, and descriptors are updated when background decoding finishes.
3. **Visibility Gating & Anti-Popin**:
   - Objects that are not fully ready (`!render_object.is_ready()`) must **not** be rendered as dummy quads or default materials.
   - Scene transitions use `set_image_auto_fade_inout` to hide asset streaming until `is_load_complete()` or `is_essential_load_complete()` triggers.
4. **Terrain Heightmap**:
   - Terrain mesh is loaded synchronously on scene open (`force_sync`) so that `CaptureHeightMap` can complete before character movement begins.
5. **Resource Manifest Sync (`resources.txt`)**:
   - `resources.txt` is updated at startup and re-synced after `initialize_application()` and before Deploy exit via `update_resource_list_file()` so that dynamically compiled shaders (`shader_caches/`) are guaranteed to be recorded even when caches are freshly generated.

### 2.3 UI & Layout Guidelines
- **2-Pass Layout System**:
  - `UIComponentData`: Logical definitions (`_pos`, `_size`, `_margin`, `_pivot`, `_pos_hint_x/y`).
  - `UIComponentInstance`: Calculated geometry (`_ui_area`, `_render_area`, `_contents_area`).
  - Do NOT apply `_pos` or `_pos_hint` directly to children inside a `BoxLayout` (use `FloatLayout` for absolute anchor/ratio positioning).
  - Set `_enable_dpi_scale` once upon widget construction, never dynamically inside frame update loops.
- **Mouse Snapping & Cursor Z-Order**:
  - When menus or dialogs open, schedule delayed gamepad snapping via `request_mouse_snap_to_widget`, and on directional key/D-Pad navigation, snap the cursor immediately via `move_mouse_to_ui_component` (guarding `touch_over` handlers with `delta != (0, 0)` so stationary snapped cursors don't override keyboard selection).
  - Always attach `CrossHairWidget` after fullscreen background/menu layers (`_game_image`, `_start_menu_widget`) and parent modal sub-popups to their host menu layer so the cursor renders on top.
- **L-Stick Separation**:
  - `WidgetNavRepeatController` only consumes D-Pad and WASD/Arrow navigation to prevent L-Stick movement from disturbing UI focus.
- **Preserve Widget Hierarchy**: Never delete or bypass intermediate container widgets (e.g. `content_layout`) when switching tabs or refactoring.
- **Raw Pointers for Widget Graphs**:
  - UI widgets intentionally use `*const` / `*mut` raw pointers (e.g., `*const InventoryWidget<'a>`) to avoid cyclic reference memory leaks and borrowing collisions in complex widget trees. Do NOT attempt to refactor them into `Rc<RefCell<T>>` or safe references unless explicitly requested.
- **Start Menu & Popup Modal Encapsulation (Stardew Valley Warm Wood/Parchment Palette)**:
  - Modal popups (e.g. Load Game, Options) host composite tab widgets (`SaveLoadWidget`, `GameOptionWidget`) inside an intermediate content container. Frame containers intercept touches via a no-op handler so clicks inside don't propagate, while clicking the backdrop dismisses the popup.
  - Startup flow transitions: `GamePhase::Start` (intro image auto fade-in) -> `GamePhase::TitleScreen` (waits for fade-in or key press) -> `GamePhase::StartMenu` (opens StartMenu with crosshair cursor enabled).
  - Menu panels, popups, and buttons use a unified Stardew Valley warm parchment/wood palette (parchment panel `(240, 222, 186)`, dark wood border `(135, 78, 42)`, warm beige button `(245, 232, 205)` with dark brown text `(75, 45, 20)`, and golden-apricot highlight `(255, 220, 145)` with orange-gold border `(255, 160, 40)` and `> text <` decorator for active focus).

---

## 3. Critical Engine & Platform Pitfalls

- **Vulkan Descriptors & Destroyed Samplers**:
  - Any texture referenced by an active descriptor set must be pinned (`entry.pin()`). Never unload or GC textures that are still bound to pipeline descriptors.
- **Window Resize / Alt+Tab (Minimize)**:
  - When minimized, `WindowEvent::Resized(0, 0)` is received. Do **not** overwrite `_window_size` with 0 (skip rendering and postpone swapchain recreation).
  - Handle `VK_SUBOPTIMAL_KHR` by setting a flag to recreate once, rather than recreating the swapchain in an infinite loop every frame.
- **Transform Previous State (Motion Blur & Camera Jitter)**:
  - Previous matrices (`_prev_*`) must be backed up strictly **once per frame** using the engine frame index (`CURRENT_FRAME`). Multiple updates per frame must not overwrite the previous frame's matrix history.
  - Camera pivot must be attached to a stable character anchor (`Character::get_camera_pivot()`), not the dynamic bounding box AABB.
- **Application Exit & Resource Cleanup**:
  - Ensure `EngineCore::terminate_application()` is invoked before window destruction to avoid Vulkan framebuffer and queue leaks upon exit.
  - When `will_terminate_application()` is flagged during `update_application()`, immediately return from `update_event_and_render_scene()` to prevent presenting unrendered swapchain images in `VK_IMAGE_LAYOUT_UNDEFINED`.

---

## 4. AI Interaction & Workflows

### 4.1 Step-by-Step Delivery
- When handling large refactorings or multi-faceted feature additions, break them down into **Phase-by-Phase milestones** (Data types → Logic → UI/Presentation).
- If the user explicitly asks for analysis or review ("코드 수정은 하지마"), do not alter files—deliver analysis only.

### 4.2 Knowledge Accumulation & Continuous Learning (MANDATORY)
- **Continuous Knowledge Accumulation**: At the end of every task or whenever a non-trivial bug/fix, architectural pattern, edge case, or user-specific coding constraint is identified:
  - You **MUST automatically append or update** the relevant section in this [AGENTS.md](file:///mnt/Workspace/StoneAge/.agents/AGENTS.md) file (or create/update a dedicated topic under `doc/` if extensive).
  - **Pruning & Brevity Principle**: Keep additions concise (1-2 bullet points). Avoid bloating the file with one-off trivial fixes. Merge or refine existing entries when concepts overlap.
  - Explicitly inform the user in the final summary of what new knowledge, pitfall, or convention was permanently recorded.
  - This guarantees that future AI sessions do not repeat mistakes and continuously build upon past discoveries.

### 4.3 Verification & Git Commits
- **Verification Workflow**:
  - Code formatting & compile checks are mandatory:
    ```bash
    cargo fmt --check
    cargo check
    ```
  - Run `cargo test --workspace` only when relevant unit/integration tests exist or when core logic/math algorithms are touched (skip for pure UI layout/visual resource tweaks if headless Vulkan device tests fail).
- **Execution commands**:
  - Linux Development: `cargo run --release -- development`
  - Shader Compile & Resource Preparation (Deploy mode): `cargo run --release -- deploy`
  - Windows cross-compilation (Wine): `./run_windows_development.sh`
- **Git Commit & Push Policy (STRICT)**:
  - **NEVER** execute `git commit` or `git push` automatically. Always leave changes uncommitted in the working tree unless explicitly instructed by the user to commit or push.
  - Follow Conventional Commits format (`feat(module): ...`, `fix(renderer): ...`).
  - **Submodule Handling (`RustEngine3D`)**: When committing is explicitly requested, commit inside `RustEngine3D/` first, then commit the updated submodule pointer in `StoneAge`.
  - **Unified Git Commit Log**: Provide suggested Git commit messages in English at the end of the response for user review/submission without executing them yourself.

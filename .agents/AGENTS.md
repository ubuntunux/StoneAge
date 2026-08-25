# Project Coding Style & Guidelines for StoneAge (RustEngine3D)

These guidelines capture the coding patterns, architectural conventions, formatting standards, and communication preferences derived from codebase analysis and interaction history.

---

## 1. Code Naming & Field Conventions

### 1.1 Struct Member Fields
- **Underscore Prefix (`_field_name`)**: All struct fields (both `pub` and private) MUST begin with an underscore prefix.
  ```rust
  pub struct InventorySlotWidget<'a> {
      pub _inventory_widget: *const InventoryWidget<'a>,
      pub _slot_index: usize,
      pub _base_contents_area_size: Vector2<f32>,
  }
  ```
- **Unused Parameters**: Prefix unused parameters or local variables with an underscore (e.g., `_window_size: &Vector2<i32>`).

### 1.2 Functions & Variables
- Use standard Rust `snake_case` for function names, methods, and local variables.
- Structs, Enums, Traits, and Type Aliases use `PascalCase`.

### 1.3 Constants & Magic Values
- **Define Explicit `const`**: Prefer defining named `const` constants (in `game_constants.rs` or module/file header) for fixed parameters, UI dimensions, layout margins, sound asset paths, texture names, and numeric literals instead of hardcoding magic numbers or string literals directly inside business/UI logic.
- **Naming**: Use `SCREAMING_SNAKE_CASE` for global, module, and local `const` definitions.
  ```rust
  pub const TOTAL_INVENTORY_SLOTS: usize = MAX_INVENTORY_ROWS * SLOTS_PER_ROW;
  pub const INVALID_ITEM_INDEX: usize = usize::MAX;
  pub const ITEM_BAR_WIDGET_POS_Y_FROM_BOTTOM: f32 = 50.0;
  ```

---

## 2. Architecture & Design Patterns

### 2.1 Service Locator Pattern
- Access managers, engine resources, audio, scene, and UI components via service locators rather than passing large context/manager instances through function parameters.
  - Examples: `get_game_ui_manager()`, `get_game_ui_manager_mut()`, `get_character_manager()`, `get_character_manager_mut()`, `get_item_manager_mut()`, `get_scene_manager()`, `get_audio_manager_mut()`, `get_engine_resources()`.
- **Avoid Parameter Bloat**: Do NOT add `&mut EngineResources` or manager instances as arguments to helper functions when service locators can provide them globally.

### 2.2 Shared Pointers & Reference Management
- Use `RcRefCell<T>`, `newRcRefCell`, `ptr_as_mut`, and `ptr_as_ref` from `rust_engine_3d::utilities::system` for safe reference borrowing and pointer operations.
- Raw pointers (e.g., `*const WidgetDefault<'a>`) are reserved for non-owning back-references or widget parent pointers.

### 2.3 UI & Component Architecture
- UI Widgets should follow standard lifecycle method naming:
  - `open_*(&mut self)` / `close_*(&mut self)` / `is_opened_*(&self) -> bool`
  - `update_*_widget(...)` / `refresh_*_widget(&mut self)` / `changed_window_size(&mut self, ...)`
- **UI Layout Properties**: When updating or creating UI components, respect layout properties (`_base_contents_area_size`, `_base_ui_size`, `_required_contents_size_hint`) and alignment (`HorizontalAlign`, `VerticalAlign`, `UILayoutType`).

---

## 3. Formatting, Error Handling & Safety

### 3.1 Formatting Guidelines
- Respect `max_width = 120` line length limit (configured in `rustfmt.toml`).
- Keep code clean, organized, and compliant with `cargo fmt`.

### 3.2 Error Handling & Panic Prevention
- Avoid blind `.unwrap()` or unchecked array/slice indexing.
- Include defensive bounds checks (e.g., `if target_slot != INVALID_ITEM_INDEX && target_slot < TOTAL_INVENTORY_SLOTS`).

---

## 4. Communication & Response Style

- **Language**: Respond in Korean (한국어).
- **Conciseness**: Keep responses clear, professional, and focused on technical delivery.
- **Clickable File Links**: Always link modified files and code symbols using markdown links with the `file://` scheme (e.g., [inventory_widget.rs](file:///mnt/Workspace/StoneAge/src/game_module/widgets/game_menu_widget/inventory_widget.rs#L25-L35)).

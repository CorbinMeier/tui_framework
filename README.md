# UI Framework

A lightweight, component-based UI framework built on top of [Ratatui](https://ratatui.rs/) for building interactive Terminal User Interfaces (TUIs) with ease.

## Key Features

- **Component-Based Architecture**: A modular system where UI elements are encapsulated into reusable components.
- **Mouse Interactivity**: Integrated support for registering and handling clickable areas during the render pass.
- **Theming System**: A flexible `Theme` provider that ensures consistent visual styling across all components.
- **Rich Component Library**: Includes pre-built components like Buttons, Text Inputs, Gauges, Tables, Lists, and Modal Dialogs.

## Core Concepts

### The `Component` Trait
The heart of the framework is the `Component` trait. Any struct implementing this trait can be rendered to a Ratatui `Frame`.

```rust
pub trait Component<S, A> {
    fn render(&self, f: &mut Frame, state: &mut S, area: Rect, context: &mut Context<A>);
}
```
- `S`: The application state.
- `A`: The Action type (typically an enum) triggered by user interactions.

### Interactivity & `Context`
Unlike standard Ratatui widgets, this framework's components use a `Context` to register their screen coordinates. This allows the application to map mouse clicks back to specific actions.

```rust
context.clickable_areas.push(ClickableArea {
    area,
    action: self.action.clone(),
});
```

### Theming
The `Theme` struct defines a palette of colors (Primary, Secondary, Accent, Success, Danger, etc.) used by all components to maintain a cohesive look and feel.

## Available Components

- **Layout & Containers**: `Panel`, `Header`, `SectionHeader`.
- **Inputs**: `TextInput` (with masking support for passwords).
- **Actions**: `Button`, `TextButton`.
- **Data Display**: `Label`, `List`, `Table`, `Gauge`.
- **Modals**: `AuthCard`, `ConfirmationDialog`.

## Usage Example

```rust
let mut clickable_areas = Vec::new();
let mut context = Context::new(&mut clickable_areas, &theme);

let button = Button::new("Click Me", MyAction::Submit);
button.render(f, &mut state, area, &mut context);

// Later, in your event loop:
if let Some(action) = context.handle_click(mouse_x, mouse_y) {
    dispatch(action);
}
```

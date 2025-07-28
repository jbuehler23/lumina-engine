# Lumina Render ECS-Driven Architecture

This document outlines the design for a generic, ECS-driven rendering architecture for the Lumina Engine. The goal is to decouple the main engine loop from the specifics of rendering, treating all rendering operations as standard ECS systems. This approach is inspired by the design of engines like Bevy and Flecs.

## Core Principles

1.  **ECS as the Single Source of Truth**: The ECS `World` holds all state. The rendering output is a direct reflection of the components and resources within the `World`.
2.  **System-Based Rendering**: All rendering logic is encapsulated within ECS systems. The main application loop does not contain any direct rendering calls.
3.  **Decoupling**: The core engine should not have any specific knowledge of *how* rendering is done. This allows for greater flexibility, such as running the engine in a headless mode without any rendering code.
4.  **Composition over Inheritance**: Different rendering passes (e.g., for the game world, UI, and debug overlays) are composed as separate systems in a schedule, rather than being part of a monolithic render pipeline.

## Key Components

### 1. `RenderContext` Resource

To avoid passing WGPU state (like `device`, `queue`, `surface`) between systems, we will store it in a global ECS resource.

-   **Name**: `RenderContext`
-   **Contents**:
    -   `wgpu::Surface`
    -   `wgpu::Device`
    -   `wgpu::Queue`
    -   `wgpu::SurfaceConfiguration`
-   **Purpose**: Provides a single, accessible point for any rendering system to get the necessary GPU handles.

### 2. Render Systems

All rendering will be performed by dedicated systems that are added to the main ECS schedule. These systems will run in a specific order at the end of each frame.

#### `game_render_system` (Future)

-   **Responsibility**: Renders the main 2D or 3D game scene.
-   **Logic**: Queries for entities with `Transform` and `Renderable` components and draws them.

#### `ui_render_system`

-   **Responsibility**: Renders the user interface.
-   **Logic**:
    1.  Queries for the `RenderContext` and `UiFramework` resources from the `World`.
    2.  Initiates a `wgpu::RenderPass`.
    3.  Calls the `ui_framework.render()` method, passing in the `RenderPass` and `Queue`.

#### `debug_render_system` (Future)

-   **Responsibility**: Renders debug information, such as collision shapes or developer console overlays.
-   **Logic**: Queries for debug-related components and resources and draws them on top of the game and UI.

### 3. The Main Application Loop

The main loop becomes extremely simple. Its only responsibilities are to run the ECS schedule and handle windowing events.

-   **Logic**:
    1.  On startup, create the `World` and register all necessary components, resources (including `RenderContext`), and systems.
    2.  Build a `Schedule` that defines the execution order of all systems for a single frame.
    3.  In the event loop, on every frame, call `schedule.run(&mut world)`.

## Data Flow

The data flow for a single frame will be as follows:

1.  **Input Systems**: Process user input and create events.
2.  **Game Logic Systems**: Mutate game state (components and resources) based on events and time.
3.  **UI Update Systems**:
    -   `ui_update_system`: Reads game state from the ECS and updates widget properties (e.g., text, progress bars).
    -   `ui_event_handler_system`: Processes UI-generated events (like `ButtonClicked`) and mutates game state.
4.  **Render Systems (in order)**:
    -   `game_render_system`: Reads `Transform` and `Renderable` components and draws the scene.
    -   `ui_render_system`: Reads the `UiFramework` resource and draws the final UI.

## Implementation Plan

To refactor the engine towards this design, the following steps are required:

1.  **Create the `RenderContext` struct** and integrate it as a resource in the `basic_ui` example.
2.  **Create a `ui_render_system`** that encapsulates the rendering logic currently in the `BasicUiApp::render` method.
3.  **Refactor the `main` function** in the `basic_ui` example to use a `Schedule` to run all systems, including the new `ui_render_system`.
4.  Ensure the main loop is clean and only responsible for running the schedule and handling window events.

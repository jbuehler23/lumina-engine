# Easy Game Creation Examples

This directory contains examples that demonstrate the vision for making game creation accessible to non-developers.

## Example 1: "My First Platformer" - 30 Minute Game

### What you'll create:
- A character that can jump and run
- Platforms to jump on
- Collectible coins
- A simple enemy that patrols
- A goal (flag) to win the level

### Steps (Future Web Editor):
1. **Choose Template**: Select "2D Platformer" from template gallery
2. **Customize Character**: Drag your sprite into the character slot
3. **Build Level**: Use tilemap brush to paint platforms
4. **Add Coins**: Drag coin prefab, set collection behavior
5. **Place Enemy**: Add patrolling enemy with visual script
6. **Set Win Condition**: Drag goal flag, configure victory screen
7. **Test & Share**: One-click test, then publish to arcade

### Current Rust Implementation:
```rust
// This shows what the generated code would look like
// Users would never see this - it's all visual!

use lumina_engine::prelude::*;

#[derive(Component)]
struct Player {
    speed: f32,
    jump_force: f32,
}

#[derive(Component)]
struct Collectible {
    points: i32,
}

fn main() -> Result<()> {
    let mut app = App::new();
    
    app.add_startup_system(setup)
       .add_system(player_movement)
       .add_system(coin_collection)
       .add_system(enemy_patrol)
       .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn player
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("player.png"),
            transform: Transform::from_xyz(0.0, 100.0, 0.0),
            ..default()
        },
        Player { speed: 200.0, jump_force: 300.0 },
        RigidBody::Dynamic,
        Collider::capsule_y(16.0, 8.0),
    ));
    
    // Spawn platforms (generated from tilemap)
    for x in 0..10 {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("platform.png"),
                transform: Transform::from_xyz(x as f32 * 64.0, 0.0, 0.0),
                ..default()
            },
            RigidBody::Fixed,
            Collider::cuboid(32.0, 16.0),
        ));
    }
    
    // Spawn collectibles
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("coin.png"),
            transform: Transform::from_xyz(200.0, 50.0, 0.0),
            ..default()
        },
        Collectible { points: 10 },
        Sensor,
    ));
}

// Visual Script: "Player Movement"
// Nodes: On Input -> Move Character -> Play Animation
fn player_movement(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Player), With<Player>>
) {
    for (mut transform, player) in query.iter_mut() {
        if input.pressed(KeyCode::A) {
            transform.translation.x -= player.speed * Time::delta_seconds();
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x += player.speed * Time::delta_seconds();
        }
        if input.just_pressed(KeyCode::Space) {
            // Jump logic (physics-based)
        }
    }
}

// Visual Script: "Coin Collection"
// Nodes: On Collision -> Add Score -> Play Sound -> Destroy Object
fn coin_collection(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    coins: Query<Entity, With<Collectible>>,
    player: Query<Entity, With<Player>>,
) {
    for event in collision_events.iter() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            let player_entity = player.single();
            
            if *e1 == player_entity && coins.contains(*e2) {
                // Play sound, add score, remove coin
                commands.entity(*e2).despawn();
            } else if *e2 == player_entity && coins.contains(*e1) {
                commands.entity(*e1).despawn();
            }
        }
    }
}
```

## Example 2: "My First Top-Down Game" - 45 Minute Game

### What you'll create:
- A character that moves in 4 directions
- Walls and obstacles
- Items to collect
- NPCs with dialogue
- Multiple rooms/levels

### Visual Script Examples:

#### Movement Script:
```
[On Input: Arrow Keys] -> [Move Character] -> [Play Walk Animation]
                      |
[No Input] -> [Stop Character] -> [Play Idle Animation]
```

#### Dialogue Script:
```
[On Collision: Player] -> [Show Dialogue Box] -> [Display Text: "Hello!"]
                                              |
                                              -> [Wait for Input] -> [Hide Dialogue]
```

#### Door/Transition Script:
```
[On Collision: Player] -> [Has Key?] -> [Yes] -> [Change Scene: "Level 2"]
                                    |
                                    -> [No] -> [Show Message: "Need a key!"]
```

## Example 3: "My First Puzzle Game" - 1 Hour Game

### Match-3 Style Game:
- Grid-based gameplay
- Drag-and-drop mechanics
- Score system
- Particle effects
- Progressive difficulty

### Visual Script Flow:
```
[On Drag Start] -> [Select Tile] -> [Highlight Possible Moves]

[On Drop] -> [Valid Move?] -> [Yes] -> [Swap Tiles] -> [Check Matches] -> [Update Score]
                           |
                           -> [No] -> [Return to Original Position]

[On Match Found] -> [Play Effect] -> [Remove Tiles] -> [Drop New Tiles] -> [Check for More Matches]
```

## Key Features That Make This Easy:

### 1. Pre-built Components
- **CharacterController2D**: Handles movement, jumping, collision
- **Collectible**: Automatic pickup detection and scoring
- **Enemy**: Basic AI patterns (patrol, chase, guard)
- **Door**: Scene transitions and unlock conditions
- **UI Elements**: Health bars, score displays, menus

### 2. Smart Defaults
- Physics automatically configured for game type
- Cameras follow player by default
- Audio plays at appropriate volumes
- Sprites automatically sized and positioned

### 3. Visual Feedback
- Instant preview of changes
- Visual indicators for interactive objects
- Automatic error detection ("Player has no way to move!")
- Performance warnings ("Too many particles may cause lag")

### 4. One-Click Features
- **Add Enemy**: Instantly creates patrolling enemy
- **Add Collectible**: Creates pickup with sound and effects
- **Add Platform**: Physics-enabled platform that character can jump on
- **Add Background**: Automatically sized and layered

## Future Web Editor Interface:

```
┌─────────────────┬─────────────────┬─────────────────┐
│  Game Objects   │    Scene View   │   Properties    │
│                 │                 │                 │
│ [+] Player      │  ┌─────────────┐ │ ┌─────────────┐ │
│ [+] Platform    │  │             │ │ │ Transform   │ │
│ [+] Enemy       │  │    [P]      │ │ │ X: 100      │ │
│ [+] Collectible │  │             │ │ │ Y: 50       │ │
│ [+] Background  │  │  ████████   │ │ │             │ │
│                 │  │             │ │ │ Sprite      │ │
│ Scripts:        │  │    [E] [C]  │ │ │ player.png  │ │
│ • Player Move   │  │             │ │ │             │ │
│ • Coin Collect  │  │             │ │ │ Collider    │ │
│ • Enemy Patrol  │  └─────────────┘ │ │ [x] Enable  │ │
│                 │                 │ └─────────────┘ │
└─────────────────┴─────────────────┴─────────────────┘
```

This approach makes game creation as simple as using a word processor, while still producing real, playable games that can be shared and enjoyed by others.
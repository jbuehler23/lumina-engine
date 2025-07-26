# Sound Assets for Lumina Engine Prototype

This directory contains sound effect descriptions and specifications for the game prototype. In a real implementation, these would be actual audio files (WAV, OGG, MP3).

## Sound Effects List

### Player Sounds
- **jump.wav** - A light "boing" sound when the player jumps (frequency: 440Hz → 880Hz, duration: 0.2s)
- **land.wav** - A soft thud when the player lands on the ground (low frequency thump, duration: 0.1s)
- **walk.wav** - Footstep sound for player movement (subtle clicking, duration: 0.15s)

### Collectible Sounds
- **coin_pickup.wav** - Classic "ding" sound when collecting coins (bell-like, frequency: 523Hz, duration: 0.3s)
- **powerup.wav** - Ascending chime for power-ups (C-E-G chord progression, duration: 0.5s)

### UI Sounds
- **button_click.wav** - Soft click for UI interactions (short tick, duration: 0.05s)
- **menu_open.wav** - Whoosh sound for opening menus (white noise sweep, duration: 0.2s)
- **menu_close.wav** - Reverse whoosh for closing menus (duration: 0.15s)

### Game Event Sounds
- **level_complete.wav** - Victory fanfare (ascending musical phrase, duration: 2.0s)
- **game_over.wav** - Descending tone for failure states (duration: 1.5s)
- **pause.wav** - Soft pause tone (single note, duration: 0.3s)

### Environment Sounds
- **ambient_forest.wav** - Background forest ambience (looped, birds and wind)
- **water_splash.wav** - Water collision sound (duration: 0.4s)

## Audio Specifications

All sounds should be:
- Sample Rate: 44.1 kHz
- Bit Depth: 16-bit
- Format: WAV (uncompressed) or OGG (compressed)
- Volume: Normalized to -6dB peak to prevent clipping

## Usage in Visual Scripts

These sounds are referenced by filename in visual script nodes:

```rust
NodeType::PlaySound("coin_pickup.wav".to_string())
```

The engine's audio system will load and play these files when triggered by visual script execution.

## Creating Audio Assets

For actual game development, use tools like:
- **Audacity** (free) - for editing and generating simple sounds
- **BFXR** or **SFXR** - for 8-bit style game sound effects
- **Freesound.org** - for downloadable sound effects (check licenses)
- **Synthesizers** - for custom musical elements

## Integration Notes

The Lumina audio system (lumina-audio crate) handles:
- Loading audio files from the assets directory
- Playing sounds triggered by visual scripts
- Managing audio channels and mixing
- 3D positional audio (future feature)
- Background music loops
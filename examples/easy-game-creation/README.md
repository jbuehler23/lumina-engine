# Easy Game Creation: The AI-Native Workflow

This directory contains examples that demonstrate the vision for making game creation accessible to non-developers using Lumina's new AI-native workflow.

## The Vision: Text-to-Game

The goal is to enable anyone to create a game by simply describing it in natural language. The AI will then generate the game world, characters, and logic automatically.

### Example Prompt:

> "Create a simple platformer game with a character that can jump and run. Add some platforms to jump on, some coins to collect, and a simple enemy that patrols back and forth. The goal is to reach a flag to win the level."

### The AI-Generated Result:

The Lumina Engine will take this prompt and generate a complete, playable game, including:

-   A player character with movement and jumping physics.
-   A series of platforms to create a level.
-   Collectible coins that disappear when touched.
-   An enemy that moves back and forth on a platform.
-   A goal flag that ends the game when reached.

## How it Works

The text-to-game process is broken down into several steps, as outlined in the [AI-Native Plan](docs/AI_NATIVE_PLAN.md):

1.  **Scene Generation:** The AI first generates a Scene Description Language (SDL) file that defines the objects in the game world.
2.  **Logic Generation:** The AI then generates visual scripts to define the behavior of the objects.
3.  **Asset Generation:** The AI can also generate assets like sprites and sounds on the fly.

## The Role of the Editor

The Lumina Editor is used to visualize and refine the AI-generated game. You can use the editor to:

-   See the game world as it's being created.
-   Select and modify objects.
-   Tweak the properties of objects.
-   Provide feedback to the AI to guide the generation process.

This new workflow makes game creation faster, easier, and more accessible than ever before. Welcome to the future of game development!

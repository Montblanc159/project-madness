# TODO

## Story
Thicken the plot

## Concept art
- find refs
- draw from refs

## DEV
### Camera
- Clamp to sides of map ??
- transition when changing level - OK
- cinematics
- 3d cam ?

### Actions
Actions based on last direction +1 gridcoords with "spacebar" - OK
Rebuild using a facing state -> allow changing orientation when on a wall - OK

### Story
- dialog sys -> OK
- story system

### Map hierarchy
- refacto colliders ? -> linked to make npc collide - OK
- refacto portals -> OK

- levels in file hierarchy for level specific logic

- dynamic spawning of zones/actionables -> insert named component on `Added<EntityInstance>`, hook on `Added<NamedComponent>` to spawn zones/actionables instead of directly hooking to `Added<EntityInstance>` => Zones/actions will now be able to be spawned by, for example, NPCs

### UI
- Dialogs comics images
- menu
- settings

### NPCs
- add npcs - OK
- make them collide - OK
- make them wander - OK
    - Slow wandering - OK
    - add a zone to limit wandering ? (clamp) - OK
- make them talk - dialogue system with ink - OK


- add ennemies (later) + [pathfinding](https://en.wikipedia.org/wiki/A*_search_algorithm)
- fight system (later)

### Add items

### Audios
- stop audios in channels when stop music message received - OK
- parameterize radius for spatial objects - OK

### Add multiple input (keyboard + gamepad)
- add player settings to change inputs dynamically

### Game states
- save system

### Optimizations
- Compressed assets ?

### Map based on madness level (later)
four levels:
- high on pills (easy mode)
- normal
- stressed (bonuses but fills easily madness level)
- mad (really hard, more monsters/hallucinations)
-> faint

### Refactors
- player
- despawn_entity_on_level_change instead of useless specific despawn - OK
- where clauses on big generic systems

### Document
Everything

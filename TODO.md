# TODO

## Story
Thicken the plot

## Concept art
- find refs
- draw from refs

## DEV
### Camera
- Clamp to sides of map ??

### Actions
Actions based on last direction +1 gridcoords with "spacebar" - OK
Rebuild using a facing state -> allow changing orientation when on a wall - OK

### Map hierarchy
- refacto colliders ? -> linked to make npc collide - OK
- refacto portals -> OK

- dynamic spawning of zones/actionables -> insert named component on `Added<EntityInstance>`, hook on `Added<NamedComponent>` to spawn zones/actionables instead of directly hooking to `Added<EntityInstance>` => Zones/actions will now be able to be spawned by, for example, NPCs

### NPCs
- add npcs - OK
- make them collide - OK
- make them wander - OK
    - Slow wandering - OK
    - add a zone to limit wandering ? (clamp)
- make them talk - dialogue system with ink


- add ennemies (later) + [pathfinding](https://en.wikipedia.org/wiki/A*_search_algorithm)
- fight system (later)

### Add items

### Add multiple input (keyboard + gamepad)


### Map based on madness level (later)
three levels:
- normal
- stressed (bonuses but fills easily level)
- mad (really hard, more monsters/hallucinations)
-> faint

### Refactors
??
- rethink some naming conventions
- game hierarchy ??

### Document
Everything
# Game Design Document

## 1 - Game Overview
**Title:** Undecided
**Genre:** Roguelike

**Game Concept:**
A turn based engine building game on a hexagonal grid. Players place and manage modules on tiles while manging an unstable central core that emits destructive waves. Tiles can flip, triggering modules effects. The goal is to keep the core stable while building an efficient engine.

## Core Gameplay Loop
1. **Place modules** on empty tiles or manage existing tiles.
2. **Wave propagation**: The central core emits a wave expanding outward one ring per turn.
3. **Tile flipping**: When the wave meets a ring, tiles flip, triggering module effects.
4. **Module activation**: Modules generate output or utility depending on their triggers.
5. **Core management**: Ensure the unstable core receives required resources before the timer ends.
6. Repeat until the core explodes (loss) or a victory condition is met.

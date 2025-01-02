# Notes

Video Link: <https://www.youtube.com/watch?v=gm7K68663rA>

Paused at around 8 mins, but just start over

---

## Overview

GOAP starts with a list of Goals

### Goals

A list of goals.

Goals        |Desired World State
-------------|-------------------
Kill Enemy   |Attacking Target X
Use Work Node|Using Node Y
Idle         |Idling

Goals

- Goals have a fixed priority (at least at their studio)
- Goals have an `IsValid()` check
  - Check to see if the goal should be pursued
- Goals have a desired world state
  - What the goal wants the AI to do.

### Actions

The AI achieves these goals through a list of Actions

Actions            |Satisfies World State     |Requires World State
-------------------|--------------------------|-----------------------------
Melee Attack       |Attacking Target X        |At Target X, Equipped Melee  
Ranged Attack      |Attacking Target X        |Near Target X, Equipped Range
Goto Target        |At Target X, Near Target X| N/A
Switch Weapon      |Equipped Z                |Idling
Play Node Animation|Using Node Y              |Equipped Melee
Goto Node          |At Node Y                 |Equipped Ranged
Idle               |Idling                    | N/A

Actions:

- Also have `IsValid()` functions
  - Ex. Goto Node Y, isValid checks if the Goto Node action can reach Node Y
    - Yes, go ahead and use this action.
    - No, not valid, don't try to use this action it won't work.
- Each action declares which world state it will satisfy.
  - Ex. Melee Attack satisfies Attacking Target X
- May require more world state.
  - Ex. Melee Attack requires At Target X and Equipped Melee

### World State

World State:

- Collection of variables: World State is a finite collection of named variables.
  - Used to communicate desire:
    - Use Node X where X is a Node
    - Be attacking character Y where Y is a character
- Desired world-state for each potential plan
- AI always has a current world state.
- Often only care about a few at a time.

### Planner

Building a plan

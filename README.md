# Filler

## Author
**Mohamed Alasfoor**

---

## Overview

**Filler** is an algorithmic strategy game where two autonomous players (robots) compete to occupy the largest possible area on a grid called the **Anfield**.  
Players take turns placing randomly generated pieces provided by a `game_engine`, following strict placement rules.

The goal is simple: **control more space than your opponent**.

---

## Game Rules

- The game is played on a 2D grid called the **Anfield**
- Two robots compete by placing pieces turn by turn
- Each piece:
  - Has a random size and shape
  - Must overlap **exactly one cell** of the player's existing territory
  - Must **not overlap** the opponent's territory
  - Must stay within the bounds of the Anfield
- If a player cannot place a piece, they must still return coordinates
- The game ends when:
  - Neither player can place a piece, or
  - A player crashes, times out, or produces invalid output

The player occupying the **largest surface area** wins.

---

## Player Representation

| Player | Active Piece | Old Pieces |
|------|--------------|------------|
| Player 1 | `a` | `@` |
| Player 2 | `s` | `$` |

Lowercase letters represent the most recently placed piece.

---

## Input / Output

### Input (STDIN)
- The engine sends:
  - Player number
  - Current Anfield
  - Current piece

### Output (STDOUT)
Your program must output the placement coordinates in the format:

```
X Y
```

If no valid move is possible, return:
```
0 0
```

---

## Example Anfield (30 x 14)

```
..............................
..............................
..$...........................
..............................
..............................
..............................
..............................
..............................
..............................
..............................
..............................
...........................@..
..............................
..............................
```

---

## Example Pieces

**Piece 2x2**
```
.#
#.
```

**Piece 5x4**
```
.##..
.##..
..#..
...#.
```

**Piece 6x3**
```
.##...
###...
#..#..
```

---

## Game Engine

The `game_engine` runs inside a Docker container and controls:
- Piece generation
- Turn handling
- Rule validation
- Scoring

### Available Flags

```
-f, -file       Path to map
-p1, -player1   Path to AI one
-p2, -player2   Path to AI two
-q, -quiet      Quiet mode
-r, -refresh    Throttling mode
-s, -seed       Random seed
-t, -time       Timeout in seconds (default: 10)
```

---

## Docker Setup

Docker is **mandatory** for this project.

### Build the Image
```
docker build -t filler .
```

### Run the Container
```
docker run -v "$(pwd)/solution":/filler/solution -it filler
```

- Your AI must be inside the `solution` directory
- The directory is mounted inside the container

### Run a Match
```
./game_engine -f maps/map01 -p1 robots/bender -p2 robots/terminator
```

---

## Project Structure

```
.
├── docker_image/
├── .dockerignore
├── .gitignore
└── README.md
```









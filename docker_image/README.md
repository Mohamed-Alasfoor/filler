# Filler (Rust)

Algorithmic 1v1 game where two bots place random pieces on the **Anfield** grid.  
Goal: cover the largest area under the placement rules.

---

## Rules
- Place pieces **in bounds**.
- Must overlap **exactly 1** of your own cells.
- **No overlap** with opponent cells.
- If stuck, output `0 0`.
- P1 symbols: `a` / `@` • P2 symbols: `s` / `$`.

---

## Build & Run (Docker)
```bash
# build image
docker build -t filler .

# run container with solution mounted
docker run -v "$(pwd)/solution":/filler/solution -it filler

# inside container, build bot
cd solution && cargo build --release

# player vs bot
./game_engine -f maps/map01 -p1 solution/target/release/solution -p2 robots/bender

# to see movements round by round
./game_engine -r -s 7 -f maps/map01 -p1 solution/target/release/solution -p2 robots/bender


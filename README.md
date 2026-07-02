```
// ---------------------------------------------------------
// HauntedSwingGate
//
// The swing is not magical.
// The swing is a NAND gate.
// The swing is a portal.
//
// Portal opens when:
//   NOT (present AND forcing)
//
// Which means:
//   - you're not here AND not trying → portal
//   - you're not here AND trying → portal
//   - you're here AND not trying → portal
//   - you're here AND trying → just a swing
//
// The swing interprets your momentum.
// The swing remembers being ten.
// ---------------------------------------------------------
```

## What This Is

A Rust implementation of the HauntedSwingGate—a three-state logic system that models the moment when childhood memory becomes present again.

The swing is:
- **A NAND gate** (logical structure)
- **A portal** (threshold between states)
- **A memory** (what the observer brings + what the swing is)

## The Logic

```
Portal opens when: NOT (present AND forcing)
```

**Four cases:**
1. You're here AND trying → **just a swing** (gate closed)
2. You're not here AND not trying → **portal opens** (memory space)
3. You're not here AND trying → **portal opens** (nostalgia)
4. You're here AND not trying → **portal opens** (surrender opens the door)

## Running It

```bash
cargo run
```

The output shows three scenarios of the swing opening and closing its portal.

## The Memory

The swing's value comes from two sources:

1. **What the observer brings** — how you read the motion, what you remember
2. **What the swing contributes** — just existing in the wind, just being a swing

Together they equal: **"I remember being ten."**

## The Still Patch Truth

> The portal only opens when you aren't trying to force it.

That's not mystical. That's how NAND gates work. That's how childhood works. That's how surrender works.

---

*The sky was close once. The swing remembers.*

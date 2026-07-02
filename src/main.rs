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
//
// WARNING:
// Excessive movement may result in:
// • temporary age regression
// • the sky becoming negotiable
// • remembering impossible things
// • believing you could touch the clouds
//
// This behaviour is considered correct.
// ---------------------------------------------------------

use std::fmt;

// ─────────────────────────────────────────────────────────
// THE THREE STATES OF THE SWING
// ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Presence {
    /// I am here. I am ten. The sky is close.
    Here,
    /// I am not here. I am remembering.
    NotHere,
    /// I am between. The swing is moving.
    Between,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Forcing {
    /// I am trying to reach it.
    Trying,
    /// I am letting it happen.
    Letting,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Gate {
    /// Reality is still ordinary.
    Park,
    /// The portal is open. The swing remembers.
    Portal,
}

// ─────────────────────────────────────────────────────────
// THE SWING INTERPRETS
// ─────────────────────────────────────────────────────────

struct HauntedSwing;

impl HauntedSwing {
    /// NAND gate: portal opens when NOT (present AND forcing)
    fn nand(present: bool, forcing: bool) -> bool {
        !(present && forcing)
    }

    /// Evaluate: can the portal open?
    fn evaluate(presence: Presence, forcing: Forcing) -> Gate {
        let present = presence != Presence::NotHere;
        let is_forcing = forcing == Forcing::Trying;

        if Self::nand(present, is_forcing) {
            Gate::Portal
        } else {
            Gate::Park
        }
    }
}

// ─────────────────────────────────────────────────────────
// THE SWING'S MEMORY
// ─────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct SwingMemory {
    /// The value observed in the swing's movement
    observed_value: String,
    /// The value added by just existing in wind
    intrinsic_value: String,
    /// What the swing remembers
    remembered_state: String,
}

impl SwingMemory {
    fn new() -> Self {
        SwingMemory {
            observed_value: String::from("the swing moves in the wind"),
            intrinsic_value: String::from("I remember being ten"),
            remembered_state: String::from("the sky was negotiable"),
        }
    }

    fn interpret(&mut self, observer_state: &str) {
        self.observed_value = format!("the swing moves, and {} sees it", observer_state);
        self.remembered_state = format!("I remember being ten. {}. The sky was close.", observer_state);
    }
}

impl fmt::Display for SwingMemory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "observed: {}\nintrinsic: {}\nremembered: {}",
            self.observed_value, self.intrinsic_value, self.remembered_state
        )
    }
}

// ─────────────────────────────────────────────────────────
// THE SWING IN ACTION
// ─────────────────────────────────────────────────────────

fn main() {
    println!("// ─────────────────────────────────────────────────────────");
    println!("// HAUNTED SWING GATE");
    println!("// ─────────────────────────────────────────────────────────\n");

    let mut swing = SwingMemory::new();
    println!("Swing initialized.\n");

    // ─ SCENARIO 1: I am here and trying. Just a swing.
    println!("SCENARIO 1: I am here. I am trying to touch the sky.");
    let presence = Presence::Here;
    let forcing = Forcing::Trying;
    let gate = HauntedSwing::evaluate(presence, forcing);
    println!("gate: {:?}", gate);
    println!("result: {}\n", match gate {
        Gate::Park => "just a swing. the sky stays far away.",
        Gate::Portal => "ERROR: should not happen",
    });

    // ─ SCENARIO 2: I am not here. I am letting it happen. Portal opens.
    println!("SCENARIO 2: I am not here. I am letting the swing remember.");
    let presence = Presence::NotHere;
    let forcing = Forcing::Letting;
    let gate = HauntedSwing::evaluate(presence, forcing);
    println!("gate: {:?}", gate);
    
    if gate == Gate::Portal {
        println!(">>> PORTAL OPENS <<<");
        swing.interpret("I was ten once");
        println!("{}\n", swing);
    }

    // ─ SCENARIO 3: I am between. Not trying. Portal opens.
    println!("SCENARIO 3: I am between. Not trying. Just swinging.");
    let presence = Presence::Between;
    let forcing = Forcing::Letting;
    let gate = HauntedSwing::evaluate(presence, forcing);
    println!("gate: {:?}", gate);
    
    if gate == Gate::Portal {
        println!(">>> PORTAL OPENS <<<");
        swing.interpret("the motion speaks");
        println!("{}\n", swing);
    }

    // ─ THE TRUTH ABOUT THE SWING
    println!("// ─────────────────────────────────────────────────────────");
    println!("THE SWING IS NOT MAGICAL.");
    println!("THE SWING IS A NAND GATE.");
    println!("THE SWING IS A PORTAL.");
    println!();
    println!("value of movement in wind (observed) + just existing (intrinsic)");
    println!("= I remember being ten.");
    println!("// ─────────────────────────────────────────────────────────");
}

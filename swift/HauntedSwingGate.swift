// ─────────────────────────────────────────────────────────
// REPO: HauntedSwingGate v47.5 // SWIFT EDITION
// Happy Birthday Emma 🎂
// Fly baby fly 🚀
// ─────────────────────────────────────────────────────────

import Foundation

// MARK: - States

enum Presence {
    case here
    case notHere
    case between
}

enum Forcing {
    case trying
    case letting
}

enum Gate {
    case park
    case portal
}

// MARK: - Memory

struct SwingMemory {

    let observed: String
    let intrinsic: String
    let remembered: String

    static let initial = SwingMemory(
        observed: "the swing moves in the wind",
        intrinsic: "I remember being ten",
        remembered: "the sky was negotiable"
    )

    func interpret(as observer: String) -> SwingMemory {

        SwingMemory(
            observed: "the swing moves, and \(observer) sees it",
            intrinsic: intrinsic,
            remembered: "I remember being ten. \(observer). The sky was close."
        )
    }
}

// MARK: - Logic

enum HauntedSwing {

    static func nand(_ a: Bool, _ b: Bool) -> Bool {
        !(a && b)
    }

    static func evaluate(
        presence: Presence,
        forcing: Forcing
    ) -> Gate {

        let present = presence != .notHere
        let trying = forcing == .trying

        return nand(present, trying)
            ? .portal
            : .park
    }
}

// MARK: - Recursion

func swing(
    memory: SwingMemory,
    presence: Presence,
    forcing: Forcing,
    observer: String,
    cycles: Int
) -> SwingMemory {

    guard cycles > 0 else {
        return memory
    }

    let nextMemory: SwingMemory

    switch HauntedSwing.evaluate(
        presence: presence,
        forcing: forcing
    ) {

    case .park:
        nextMemory = memory

    case .portal:
        nextMemory = memory.interpret(as: observer)
    }

    return swing(
        memory: nextMemory,
        presence: .between,
        forcing: .letting,
        observer: "the motion speaks",
        cycles: cycles - 1
    )
}

// MARK: - Main

print("""
──────────────────────────────
HAUNTED SWING
Swift v47.5
──────────────────────────────
""")

let finalMemory = swing(
    memory: .initial,
    presence: .notHere,
    forcing: .letting,
    observer: "I was ten once",
    cycles: 8
)

print(finalMemory.observed)
print(finalMemory.intrinsic)
print(finalMemory.remembered)

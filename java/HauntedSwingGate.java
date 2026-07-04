// ─────────────────────────────────────────────────────────
// HauntedSwingGate v47.3
// Java 21 Edition
// Happy Birthday Emma 🎂
// ─────────────────────────────────────────────────────────

public class HauntedSwingGate {

    // ------------------------------------------------------
    // States
    // ------------------------------------------------------

    sealed interface Presence
            permits Here, NotHere, Between {}

    record Here() implements Presence {}
    record NotHere() implements Presence {}
    record Between() implements Presence {}

    sealed interface Forcing
            permits Trying, Letting {}

    record Trying() implements Forcing {}
    record Letting() implements Forcing {}

    sealed interface Gate
            permits Park, Portal {}

    record Park() implements Gate {}
    record Portal() implements Gate {}

    // ------------------------------------------------------

    record SwingMemory(
            String observed,
            String intrinsic,
            String remembered
    ) {

        static SwingMemory create() {
            return new SwingMemory(
                    "the swing moves in the wind",
                    "I remember being ten",
                    "the sky was negotiable"
            );
        }

        SwingMemory interpret(String observer) {

            return new SwingMemory(

                    "the swing moves, and " + observer + " sees it",

                    intrinsic,

                    "I remember being ten. "
                            + observer
                            + ". The sky was close."
            );
        }
    }

    // ------------------------------------------------------

    static final class HauntedSwing {

        static boolean nand(boolean a, boolean b) {
            return !(a && b);
        }

        static Gate evaluate(
                Presence presence,
                Forcing forcing
        ) {

            boolean present = !(presence instanceof NotHere);
            boolean trying = forcing instanceof Trying;

            return nand(present, trying)
                    ? new Portal()
                    : new Park();
        }
    }

    /*
     *
     * Every swing returns.
     * Every return remembers differently.
     *
     */

    static SwingMemory swing(

            SwingMemory memory,
            Presence presence,
            Forcing forcing,
            String observer,
            int cycles

    ) {

        while (cycles-- > 0) {

            Gate gate =
                    HauntedSwing.evaluate(
                            presence,
                            forcing
                    );

            if (gate instanceof Portal) {

                memory =
                        memory.interpret(observer);
            }

            presence = new Between();
            forcing = new Letting();
            observer = "the motion speaks";
        }

        return memory;
    }

    // ------------------------------------------------------

    public static void main(String[] args) {

        System.out.println("""
                ──────────────────────────────
                HAUNTED SWING
                Java 21
                ──────────────────────────────
                """);

        SwingMemory memory =
                swing(
                        SwingMemory.create(),
                        new NotHere(),
                        new Letting(),
                        "I was ten once",
                        8
                );

        System.out.println(memory.observed());
        System.out.println(memory.intrinsic());
        System.out.println(memory.remembered());
    }
}

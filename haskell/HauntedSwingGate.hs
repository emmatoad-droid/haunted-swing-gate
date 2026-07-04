-- ─────────────────────────────────────────────────────────
-- HauntedSwingGate v47.5
-- Haskell Edition
-- Happy Birthday Emma 🎂
-- Fly baby fly 🚀
-- ─────────────────────────────────────────────────────────

module Main where

-- ------------------------------------------------------
-- States
-- ------------------------------------------------------

data Presence
    = Here
    | NotHere
    | Between
    deriving (Eq, Show)

data Forcing
    = Trying
    | Letting
    deriving (Eq, Show)

data Gate
    = Park
    | Portal
    deriving (Eq, Show)

-- ------------------------------------------------------

data SwingMemory = SwingMemory
    { observed   :: String
    , intrinsic  :: String
    , remembered :: String
    } deriving Show

createMemory :: SwingMemory
createMemory =
    SwingMemory
        "the swing moves in the wind"
        "I remember being ten"
        "the sky was negotiable"

interpret :: SwingMemory -> String -> SwingMemory
interpret memory observer =
    memory
        { observed =
            "the swing moves, and "
            ++ observer
            ++ " sees it"

        , remembered =
            "I remember being ten. "
            ++ observer
            ++ ". The sky was close."
        }

-- ------------------------------------------------------
-- Logic
-- ------------------------------------------------------

nandGate :: Bool -> Bool -> Bool
nandGate a b = not (a && b)

evaluate :: Presence -> Forcing -> Gate
evaluate presence forcing

    | nandGate present trying = Portal
    | otherwise               = Park

  where
    present = presence /= NotHere
    trying  = forcing == Trying

{-|
 Every swing returns.

 Every return remembers differently.
-}

swing
    :: SwingMemory
    -> Presence
    -> Forcing
    -> String
    -> Int
    -> SwingMemory

swing memory _ _ _ 0 = memory

swing memory presence forcing observer cycles =

    swing
        nextMemory
        Between
        Letting
        "the motion speaks"
        (cycles - 1)

  where

    nextMemory =
        case evaluate presence forcing of

            Park ->
                memory

            Portal ->
                interpret memory observer

-- ------------------------------------------------------

main :: IO ()

main = do

    putStrLn "──────────────────────────────"
    putStrLn "HAUNTED SWING"
    putStrLn "Haskell"
    putStrLn "──────────────────────────────"

    let finalMemory =
            swing
                createMemory
                NotHere
                Letting
                "I was ten once"
                8

    putStrLn ""

    putStrLn (observed finalMemory)
    putStrLn (intrinsic finalMemory)
    putStrLn (remembered finalMemory)

-- ─────────────────────────────────────────────────────────
-- HauntedGarden v1.0
-- The garden remembers.
-- Happy Birthday Emma 🌸
-- ─────────────────────────────────────────────────────────

module Main where

-- --------------------------------------------------------
-- World
-- --------------------------------------------------------

data World = World
    { flowers :: Int
    , bees    :: Int
    , fruit   :: Int
    } deriving Show

-- --------------------------------------------------------
-- Haunting
-- --------------------------------------------------------

data Haunting
    = GentleSpirit
    | TricksterSpirit
    | SilentWind
    deriving Show

-- --------------------------------------------------------
-- Observer
-- --------------------------------------------------------

data Observer = Observer
    { name        :: String
    , interpretation :: String
    } deriving Show

-- --------------------------------------------------------
-- Memory
-- --------------------------------------------------------

data Memory = Memory
    { remembered :: [String]
    } deriving Show

-- --------------------------------------------------------
-- Garden
-- --------------------------------------------------------

data Garden = Garden
    { world     :: World
    , haunting  :: Haunting
    , observer  :: Observer
    , memory    :: Memory
    } deriving Show

-- --------------------------------------------------------
-- Hidden influence
-- --------------------------------------------------------

haunt :: Haunting -> World -> World

haunt GentleSpirit w =
    w { flowers = flowers w + 2 }

haunt TricksterSpirit w =
    w { fruit = max 0 (fruit w - 1) }

haunt SilentWind w =
    w { bees = bees w + 1 }

-- --------------------------------------------------------
-- Observer constructs meaning
-- --------------------------------------------------------

observe :: Observer -> Haunting -> String

observe person GentleSpirit =
    name person ++
    " smiles. The garden feels welcoming."

observe person TricksterSpirit =
    name person ++
    " frowns. Something feels mischievous."

observe person SilentWind =
    name person ++
    " blames the wind."

-- --------------------------------------------------------
-- Memory grows
-- --------------------------------------------------------

remember :: Garden -> Garden

remember garden =

    garden
        { memory =
            Memory $
                remembered (memory garden)
                ++ [observe (observer garden) (haunting garden)]
        }

-- --------------------------------------------------------
-- One cycle
-- --------------------------------------------------------

cycleGarden :: Garden -> Garden

cycleGarden garden =

    remember $

        garden
            { world =
                haunt
                    (haunting garden)
                    (world garden)
            }

-- --------------------------------------------------------
-- Main
-- --------------------------------------------------------

main :: IO ()

main = do

    let garden =
            Garden
                (World 3 2 1)
                GentleSpirit
                (Observer "Emma" "")
                (Memory [])

    let final = iterate cycleGarden garden !! 5

    print (world final)

    putStrLn ""

    mapM_ putStrLn
        (remembered (memory final))

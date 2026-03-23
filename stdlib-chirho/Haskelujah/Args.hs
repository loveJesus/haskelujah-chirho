-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16

-- | Built-in command-line argument parsing. No package install needed.
module Haskelujah.Args (
    getArgs,
    getFlag,
    getOption,
    getPositional,
) where

import System.Environment (getArgs)

-- | Check if a flag is present (e.g. --verbose).
getFlag :: String -> [String] -> Bool
getFlag flag args = ("--" ++ flag) `elem` args

-- | Get the value of an option (e.g. --output foo).
getOption :: String -> [String] -> Maybe String
getOption _ [] = Nothing
getOption opt (x:y:rest)
    | x == "--" ++ opt = Just y
    | otherwise = getOption opt (y:rest)
getOption _ [_] = Nothing

-- | Get positional arguments (non-flag, non-option).
getPositional :: [String] -> [String]
getPositional [] = []
getPositional (('-':'-':_):_:rest) = getPositional rest
getPositional (('-':'-':_):rest) = getPositional rest
getPositional (x:rest) = x : getPositional rest

-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16

-- | Built-in process/shell execution. No package install needed.
module Haskelujah.Process (
    shell,
    exec,
    ExitCode(..),
) where

import System.Exit (ExitCode(..))

-- | Run a shell command and return the output.
shell :: String -> IO String
shell cmd = do
    putStrLn ("$ " ++ cmd)
    return "" -- placeholder — will use System.Process FFI

-- | Execute a command with arguments.
exec :: String -> [String] -> IO ExitCode
exec prog args = do
    putStrLn ("exec: " ++ prog ++ " " ++ unwords args)
    return ExitSuccess -- placeholder

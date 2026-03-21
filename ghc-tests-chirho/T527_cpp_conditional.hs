{-# LANGUAGE CPP #-}
-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
-- TEST: compile_and_run
-- EXPECTED: Modern GHC\nbase available
module Main where

#if __GLASGOW_HASKELL__ >= 800
greeting :: String
greeting = "Modern GHC"
#else
greeting :: String
greeting = "Old GHC"
#endif

#ifdef MIN_VERSION_base
baseMsg :: String
baseMsg = "base available"
#else
baseMsg :: String
baseMsg = "no base"
#endif

main :: IO ()
main = do
  putStrLn greeting
  putStrLn baseMsg

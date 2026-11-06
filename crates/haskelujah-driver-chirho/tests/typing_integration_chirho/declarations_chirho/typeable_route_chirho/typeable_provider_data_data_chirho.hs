-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

import Data.Data (Typeable)

kindChirho :: Typeable a => a -> String
kindChirho _ = "typeable"

main :: IO ()
main = putStrLn (kindChirho True)

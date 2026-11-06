-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

import Prelude
import qualified Prelude as P (Real(..))

main :: IO ()
main = print (P.toRational (3 :: Int))

-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

import Prelude hiding (Show, show)
import qualified Prelude

-- The program's own class, spelled like the Prelude's. Its Int instance answers by name.
class Show a where
  show :: a -> String

instance Show Int where
  show _ = "source"

-- The compiler's Show at a list of Maybe values.
main :: IO ()
main = putStrLn (Prelude.show [Just (1 :: Int)])

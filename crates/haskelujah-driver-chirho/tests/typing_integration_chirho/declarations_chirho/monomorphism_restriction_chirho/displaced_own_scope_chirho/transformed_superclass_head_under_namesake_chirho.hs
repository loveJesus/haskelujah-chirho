-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

import qualified Data.Typeable as R

-- The program's own class, spelled like the compiler's Typeable.
class Typeable t where
  describeChirho :: t -> String

instance Typeable (Maybe a) where
  describeChirho _ = "maybe"

-- Its superclass is the compiler's Typeable at a transformed head, `Maybe a`, not `a`: the
-- instance below needs `Typeable (Maybe Int)`, never a dictionary at the child's key `Int`.
class R.Typeable (Maybe a) => WeirdChirho a where
  weirdChirho :: a -> String

instance WeirdChirho Int where
  weirdChirho _ = "weird"

main :: IO ()
main = putStrLn (weirdChirho (1 :: Int))

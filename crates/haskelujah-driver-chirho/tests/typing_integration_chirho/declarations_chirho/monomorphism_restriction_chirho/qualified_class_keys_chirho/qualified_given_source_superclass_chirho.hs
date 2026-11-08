-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

import Prelude hiding (Num)
import qualified Prelude

-- The program's own class, spelled like the Prelude's, has Show as its superclass. The
-- Prelude's Num has no superclass.
class Show a => Num a where
  sizeChirho :: a -> Int

-- The given is the Prelude's Num, whose own record has no superclass, so nothing gives Show b.
describeChirho :: Prelude.Num b => b -> String
describeChirho valueChirho = show valueChirho

main :: IO ()
main = putStrLn (describeChirho (1 :: Int))

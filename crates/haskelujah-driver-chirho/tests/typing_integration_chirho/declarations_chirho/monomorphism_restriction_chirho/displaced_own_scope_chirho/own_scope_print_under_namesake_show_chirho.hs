-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

import Prelude hiding (Show, show)
import qualified Prelude

-- The program's own class, spelled like the Prelude's. Its Int instance answers by name.
class Show a where
  show :: a -> String

instance Show Int where
  show _ = "source"

-- The Prelude's print at the binding's own compiler Show dictionary.
reportChirho :: Prelude.Show b => b -> IO ()
reportChirho valueChirho = print valueChirho

main :: IO ()
main = do
  reportChirho (7 :: Int)
  reportChirho [True, False]

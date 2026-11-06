-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

import Prelude hiding (print)
import qualified Prelude as P

print :: String -> IO ()
print textChirho = putStrLn ("own " ++ textChirho)

showAllChirho :: Show aChirho => [aChirho] -> IO ()
showAllChirho valuesChirho = mapM_ P.print valuesChirho

main :: IO ()
main = do
  print "hello"
  mapM_ print ["x", "y"]
  showAllChirho "ab"

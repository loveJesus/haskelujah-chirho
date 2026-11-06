-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

twiceChirho :: Monad mChirho => mChirho () -> mChirho ()
twiceChirho actionChirho = actionChirho >> actionChirho

sevenChirho :: Monad mChirho => mChirho Int
sevenChirho = return 3 >>= \countChirho -> return (countChirho + 4)

relayChirho :: Monad mChirho => mChirho String -> (String -> mChirho ()) -> mChirho ()
relayChirho readChirho writeChirho = readChirho >>= writeChirho

greetChirho :: Monad mChirho => (String -> mChirho ()) -> mChirho Int
greetChirho sayChirho = do
  sayChirho "hello"
  sayChirho "again"
  return 2

showBothChirho :: Show aChirho => aChirho -> aChirho -> IO ()
showBothChirho leftChirho rightChirho = print leftChirho >> print rightChirho

main :: IO ()
main = do
  twiceChirho (putStrLn "tick")
  sevenChirho >>= print
  relayChirho (return "relay") putStrLn
  greetChirho putStrLn >>= print
  countChirho <- sevenChirho
  print (countChirho * 2)
  showBothChirho 'a' 'b'
  putStrLn "done" >> putStrLn "end"

-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where
main = do
  let inputChirho = getLine
  inputChirho `seq` putStrLn "before-chirho"
  firstChirho <- inputChirho
  secondChirho <- inputChirho
  putStrLn (firstChirho ++ "/" ++ secondChirho)

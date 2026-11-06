-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DerivingStrategies, GeneralizedNewtypeDeriving #-}
newtype BoxChirho aChirho = BoxChirho aChirho deriving newtype (Show)
{-# NOINLINE renderChirho #-}
renderChirho :: Show aChirho => aChirho -> String
renderChirho valueChirho = show valueChirho
main :: IO ()
main = do
  putStrLn (renderChirho (BoxChirho (7 :: Int)))
  putStrLn (renderChirho (BoxChirho True))

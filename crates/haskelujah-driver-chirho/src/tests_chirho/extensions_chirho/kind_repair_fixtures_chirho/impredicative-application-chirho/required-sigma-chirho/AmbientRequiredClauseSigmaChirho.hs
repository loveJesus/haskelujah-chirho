-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RequiredTypeArguments, RankNTypes #-}
module Main where
compareChirho :: Ord bChirho => bChirho -> forall aChirho -> Ord aChirho => aChirho -> aChirho -> Bool
compareChirho _ typeChirho leftChirho rightChirho = leftChirho < rightChirho
main :: IO ()
main = print (compareChirho 'x' Double (-2.5) 1.0, compareChirho (42 :: Int) Char 'a' 'b')

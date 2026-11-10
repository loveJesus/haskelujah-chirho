-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RequiredTypeArguments, RankNTypes #-}
module Main where
compareChirho :: forall aChirho -> Ord aChirho => aChirho -> aChirho -> Bool
compareChirho typeChirho leftChirho rightChirho = leftChirho < rightChirho
main :: IO ()
main = print (compareChirho Double (-2.5) 1.0, compareChirho Char 'a' 'b')

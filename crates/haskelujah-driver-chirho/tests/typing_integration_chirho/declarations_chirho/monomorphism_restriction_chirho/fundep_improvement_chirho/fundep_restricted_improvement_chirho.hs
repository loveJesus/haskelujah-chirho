-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE FunctionalDependencies #-}
module Main where

-- The class fundep makes the result a function of the argument.
class DescribeChirho aChirho bChirho | aChirho -> bChirho where
  describeChirho :: aChirho -> bChirho

instance DescribeChirho Bool Int where
  describeChirho flagChirho = if flagChirho then 42 else 0

instance DescribeChirho Char String where
  describeChirho letterChirho = [letterChirho, letterChirho]

-- Restricted bindings (no signature, no arguments): only the fundep fixes each result type.
answerChirho = describeChirho True
echoChirho = describeChirho 'x'

main :: IO ()
main = do
  print (answerChirho + 1)
  putStrLn echoChirho

-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, PolyKinds, TypeFamilies, TypeApplications, TypeAbstractions, StandaloneKindSignatures #-}
module Main where
import Data.Kind (Type)

data CellChirho (valueChirho :: kindChirho) = CellChirho Int
type AliasChirho (valueChirho :: kindChirho) = CellChirho valueChirho
type ChainChirho (valueChirho :: kindChirho) = AliasChirho valueChirho
type PartialChirho = CellChirho

readAliasChirho :: ChainChirho True -> Int
readAliasChirho (CellChirho valueChirho) = valueChirho
readPartialChirho :: PartialChirho False -> Int
readPartialChirho (CellChirho valueChirho) = valueChirho
givenChirho :: (CellChirho valueChirho ~ AliasChirho Int) => valueChirho -> Int
givenChirho valueChirho = valueChirho

type PhantomChirho :: forall kindChirho. Type
data PhantomChirho @kindChirho = PhantomChirho
type PhantomAliasChirho kindChirho = PhantomChirho @kindChirho
preserveChirho :: PhantomAliasChirho Bool -> PhantomChirho @Bool
preserveChirho valueChirho = valueChirho

main :: IO ()
main = do
  print (readAliasChirho (CellChirho 42))
  print (readPartialChirho (CellChirho 7))
  print (givenChirho (11 :: Int))

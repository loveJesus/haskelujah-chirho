-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, TypeFamilies #-}
module Main where
import Data.Kind (Type)
data BoxChirho (fChirho :: kChirho -> Type) (aChirho :: kChirho) = BoxChirho (fChirho aChirho)
type family BaseChirho valueChirho :: Type -> Type
type instance BaseChirho Int = BoxChirho Maybe
valueChirho :: BaseChirho Int Bool
valueChirho = BoxChirho (Just True)
main = case valueChirho of BoxChirho (Just answerChirho) -> print answerChirho

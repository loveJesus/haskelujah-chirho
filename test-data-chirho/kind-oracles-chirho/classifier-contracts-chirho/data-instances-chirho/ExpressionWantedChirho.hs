-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, GADTs, KindSignatures, ScopedTypeVariables, TypeFamilies #-}
module Main where
import Data.Kind (Type)
data ProxyChirho aChirho = ProxyChirho
type family IndexChirho aChirho :: Bool
type instance IndexChirho Int = 'True
type instance IndexChirho Bool = 'False
data family WitnessChirho (bChirho :: Bool)
data instance WitnessChirho bChirho where
  YesChirho :: WitnessChirho 'True
  NoChirho :: WitnessChirho 'False
keepChirho :: forall aChirho. ProxyChirho aChirho -> WitnessChirho (IndexChirho aChirho) -> WitnessChirho (IndexChirho aChirho)
keepChirho _ _ = YesChirho
answerChirho :: WitnessChirho bChirho -> Int
answerChirho YesChirho = 1
answerChirho NoChirho = 2
main = do
  print (answerChirho (keepChirho (ProxyChirho :: ProxyChirho Int) YesChirho))
  print (answerChirho (keepChirho (ProxyChirho :: ProxyChirho Bool) NoChirho))

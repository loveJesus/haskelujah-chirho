-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, PolyKinds, StandaloneKindSignatures, TypeFamilies, GADTs #-}
module FamilyIndexedClassifierChirho where
import Data.Kind (Type)
data IndexChirho = OnlyChirho
data WitnessChirho :: IndexChirho -> Type where
  OnlyWitnessChirho :: WitnessChirho OnlyChirho
type family ClassifierChirho (indexChirho :: IndexChirho) :: Type where
  ClassifierChirho OnlyChirho = Type
type SelectChirho :: WitnessChirho indexChirho -> ClassifierChirho indexChirho -> Type
type family SelectChirho witnessChirho valueChirho where
  SelectChirho OnlyWitnessChirho valueChirho = valueChirho

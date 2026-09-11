-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, PolyKinds, StandaloneKindSignatures, TypeFamilies #-}
module FamilyRowClassifiersChirho where
import Data.Kind (Type)
type ChooseChirho :: forall kindChirho. kindChirho -> Type
type family ChooseChirho valueChirho where
  ChooseChirho Int = Int
  ChooseChirho 'False = Char
valueChirho :: ChooseChirho 'False
valueChirho = 'x'

-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, RankNTypes, StandaloneKindSignatures, TypeFamilies #-}
module FamilyStandaloneHeadChirho where
import Data.Kind (Type)
type SignatureChirho = forall (kindChirho :: Type) -> kindChirho -> kindChirho
type InhabitantChirho :: SignatureChirho
data family InhabitantChirho kindChirho valueChirho
type ConsumerChirho :: SignatureChirho -> Type
newtype ConsumerChirho familyChirho = ConsumerChirho (familyChirho Type Bool)
type AppliedChirho = ConsumerChirho InhabitantChirho

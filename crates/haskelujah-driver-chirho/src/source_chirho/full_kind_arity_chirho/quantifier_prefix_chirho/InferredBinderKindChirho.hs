-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE StandaloneKindSignatures, PolyKinds, KindSignatures, ExplicitForAll #-}
module InferredBinderKindChirho where
import Data.Kind (Type)
type PolyChirho :: forall {kChirho :: Type}. kChirho -> Type
data PolyChirho (aChirho :: kChirho) = PolyChirho

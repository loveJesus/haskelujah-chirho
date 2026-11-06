-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, MultiParamTypeClasses, TypeAbstractions, KindSignatures, StandaloneKindSignatures, ExplicitForAll #-}
module ScopedClassVisibilityChirho where
import Data.Proxy (Proxy)
import Data.Kind (Constraint)
type SizedChirho :: forall kChirho. kChirho -> Constraint
class SizedChirho @kChirho (aChirho :: kChirho) where
  sizeChirho :: Proxy aChirho -> Int
type PairChirho :: forall kaChirho kbChirho. kaChirho -> kbChirho -> Constraint
class PairChirho @kaChirho @kbChirho (aChirho :: kaChirho) (bChirho :: kbChirho) where
  pairChirho :: Proxy aChirho -> Proxy bChirho -> Int

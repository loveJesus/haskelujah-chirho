-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, TypeOperators, TypeFamilies #-}
module AssociatedDefaultChirho where
import Data.Kind (Type)
import Data.Proxy (Proxy)
data PairChirho aChirho bChirho = aChirho :*: bChirho
class PairClassChirho (aChirho :: Type) where
  type PairOfChirho aChirho :: PairChirho Type Type
  type PairOfChirho aChirho = '(AssociatedDefaultChirho.:*:) aChirho Bool
instance PairClassChirho Int
pairChirho :: Proxy (PairOfChirho Int) -> Proxy (Int ':*: Bool)
pairChirho valueChirho = valueChirho

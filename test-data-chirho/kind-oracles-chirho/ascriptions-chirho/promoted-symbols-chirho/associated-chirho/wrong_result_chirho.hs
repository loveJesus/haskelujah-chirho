-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, TypeOperators, TypeFamilies #-}
module WrongAssociatedResultChirho where
import Data.Kind (Type)
import Data.Proxy (Proxy)
data PairChirho aChirho bChirho = aChirho :*: bChirho
class PairClassChirho aChirho where
  type PairOfChirho aChirho :: PairChirho Type Type
instance PairClassChirho Int where
  type PairOfChirho Int = '(WrongAssociatedResultChirho.:*:) Int Bool
pairChirho :: Proxy (PairOfChirho Int) -> Proxy (Bool ':*: Int)
pairChirho valueChirho = valueChirho

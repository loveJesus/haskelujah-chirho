-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, TypeOperators, TypeFamilies, FlexibleInstances #-}
module AssociatedPatternChirho where
import Data.Kind (Type)
data PairChirho aChirho bChirho = aChirho :*: bChirho
class FirstClassChirho (pairChirho :: PairChirho Type Type) where
  type FirstChirho pairChirho :: Type
instance FirstClassChirho (leftChirho ':*: rightChirho) where
  type FirstChirho ('(AssociatedPatternChirho.:*:) leftChirho rightChirho) = leftChirho
firstChirho :: FirstChirho (Int ':*: Bool) -> Int
firstChirho valueChirho = valueChirho

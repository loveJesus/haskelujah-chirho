-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, TypeOperators, TypeFamilies #-}
module QualifiedFamilyChirho where
import Data.Kind (Type)
data PairChirho aChirho bChirho = aChirho :*: bChirho
type family FirstChirho (pairChirho :: PairChirho Type Type) :: Type where
  FirstChirho ('(QualifiedFamilyChirho.:*:) leftChirho rightChirho) = leftChirho
firstChirho :: FirstChirho (Int ':*: Bool) -> Int
firstChirho valueChirho = valueChirho

-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, GADTs, KindSignatures #-}
module WrongKindChirho where
import Data.Kind (Type)
data family FamilyChirho (aChirho :: Type)
data instance FamilyChirho Maybe where
  BadChirho :: FamilyChirho Maybe

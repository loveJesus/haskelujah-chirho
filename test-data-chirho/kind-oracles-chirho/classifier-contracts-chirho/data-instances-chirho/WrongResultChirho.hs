-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, GADTs #-}
module WrongResultChirho where
data family FamilyChirho aChirho
data instance FamilyChirho Int where
  BadChirho :: FamilyChirho Bool

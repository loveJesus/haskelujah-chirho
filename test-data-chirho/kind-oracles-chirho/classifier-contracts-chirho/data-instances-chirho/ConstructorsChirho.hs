-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, GADTs #-}
module Main where
data family FamilyChirho aChirho
data instance FamilyChirho Int = IntChirho Int
data instance FamilyChirho Bool where
  BoolChirho :: Bool -> FamilyChirho Bool
intChirho :: FamilyChirho Int -> Int
intChirho (IntChirho nChirho) = nChirho
boolChirho :: FamilyChirho Bool -> Bool
boolChirho (BoolChirho bChirho) = bChirho
main = do
  print (intChirho (IntChirho 7))
  print (boolChirho (BoolChirho True))

-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, TypeFamilies #-}
module FamilyEquationScopeChirho where

data PairChirho = PairChirho Bool Bool
type family SecondChirho (aChirho :: PairChirho) :: Bool where
  SecondChirho ('PairChirho _ aChirho) = aChirho

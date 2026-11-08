-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, RankNTypes, TypeApplications, TypeAbstractions, ScopedTypeVariables #-}
module ConstructorGuardScopeChirho where
data BoxChirho aChirho where
  BoxChirho :: forall aChirho. aChirho -> BoxChirho aChirho
outerChirho :: forall aChirho. aChirho -> BoxChirho Bool -> aChirho
outerChirho valueChirho boxChirho
  | BoxChirho @aChirho innerChirho <- boxChirho = valueChirho
  | otherwise = (valueChirho :: aChirho)

-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, RankNTypes, TypeApplications, TypeAbstractions, ScopedTypeVariables #-}
module ConstructorListScopeChirho where
data BoxChirho aChirho where
  BoxChirho :: forall aChirho. aChirho -> BoxChirho aChirho
outerChirho :: forall aChirho. aChirho -> aChirho
outerChirho valueChirho = let valuesChirho = [(innerChirho :: aChirho) | BoxChirho @aChirho innerChirho <- [BoxChirho True]]
                         in (valueChirho :: aChirho)

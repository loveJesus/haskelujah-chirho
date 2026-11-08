-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, RankNTypes, TypeApplications, TypeAbstractions, ScopedTypeVariables #-}
module ConstructorDoShadowChirho where
data BoxChirho aChirho where
  BoxChirho :: forall aChirho. aChirho -> BoxChirho aChirho
outerChirho :: forall aChirho. aChirho -> aChirho
outerChirho valueChirho = let actionChirho = do
                               BoxChirho @aChirho innerChirho <- Just (BoxChirho True)
                               pure (innerChirho :: aChirho)
                         in (valueChirho :: aChirho)

-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RequiredTypeArguments, TypeAbstractions, RankNTypes, ScopedTypeVariables, PolyKinds, DataKinds, GADTs, KindSignatures, TypeFamilies, TypeOperators, FlexibleContexts #-}
{-# LANGUAGE TypeFamilyDependencies #-}
module NoninjectiveFamilyAmbiguousChirho where
type family FChirho aChirho
fChirho :: forall aChirho bChirho. (FChirho aChirho ~ bChirho) => bChirho -> bChirho
fChirho xChirho = xChirho

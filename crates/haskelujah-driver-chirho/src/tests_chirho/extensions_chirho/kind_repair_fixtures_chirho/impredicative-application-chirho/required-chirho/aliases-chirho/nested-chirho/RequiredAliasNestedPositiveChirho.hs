-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RequiredTypeArguments, TypeAbstractions, RankNTypes, ScopedTypeVariables #-}
module RequiredAliasNestedPositiveChirho where
type IdentityChirho = forall aChirho -> aChirho -> aChirho
consumeChirho :: forall aChirho -> Int
consumeChirho _ = 0
outerChirho :: IdentityChirho
outerChirho tChirho (xChirho :: tChirho) = let { innerChirho :: IdentityChirho; innerChirho uChirho (yChirho :: uChirho) = let { ignoredChirho = consumeChirho tChirho + consumeChirho uChirho } in yChirho } in innerChirho tChirho xChirho

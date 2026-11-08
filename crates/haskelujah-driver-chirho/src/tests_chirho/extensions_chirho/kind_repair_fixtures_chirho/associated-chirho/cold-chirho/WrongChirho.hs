-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, TypeFamilies, FlexibleInstances #-}
module WrongChirho where
import OriginChirho (CChirho, AliasChirho, TagChirho)
instance CChirho Maybe
witnessChirho :: AliasChirho -> TagChirho Bool
witnessChirho x = x

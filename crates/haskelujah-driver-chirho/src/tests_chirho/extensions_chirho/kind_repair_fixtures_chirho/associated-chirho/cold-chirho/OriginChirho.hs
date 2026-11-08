-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, KindSignatures, TypeFamilies #-}
module OriginChirho (CChirho, AliasChirho, TagChirho) where
data TagChirho (a :: k) = TagChirho
class CChirho (a :: k) where
  type FChirho a :: k
  type FChirho a = a
type AliasChirho = TagChirho (FChirho Maybe)

-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
module ALibChirho (AliasChirho, mkChirho, useChirho) where
data TokenChirho = TokenChirho
type AliasChirho = TokenChirho
mkChirho :: AliasChirho
mkChirho = TokenChirho
useChirho :: AliasChirho -> Char
useChirho _ = 'a'

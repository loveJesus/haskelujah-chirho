-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
module UserNoInlineChirho where
fChirho xChirho = xChirho
{-# NOINLINE [2] fChirho #-}
resultChirho = fChirho 'p'

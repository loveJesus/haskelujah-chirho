-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
module MultipleGuardEquationsChirho where
import GHC.Internal.Classes (ltInt)
fChirho nChirho | nChirho `ltInt` 0 = 'n'
fChirho _ = 'p'
resultChirho = fChirho (5 :: Int)

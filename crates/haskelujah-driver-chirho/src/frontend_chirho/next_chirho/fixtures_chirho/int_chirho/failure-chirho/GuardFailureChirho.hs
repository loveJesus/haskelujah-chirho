-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
module GuardFailureChirho where
import GHC.Internal.Classes (ltInt)
fChirho :: Int -> Char
fChirho nChirho | nChirho `ltInt` 0 = 'n'
resultChirho = fChirho 5

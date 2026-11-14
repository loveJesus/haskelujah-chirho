-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
module GuardOrderChirho where
import GHC.Internal.Classes (ltInt)
fChirho :: Int -> Char
fChirho nChirho | nChirho `ltInt` 0 = 'n'
                | otherwise = 'p'
boomBoolChirho = case True of False -> False
firstChirho :: Int -> Char
firstChirho _ | True = 'f'
             | boomBoolChirho = 'x'
resultChirho = fChirho 5
negativeChirho = fChirho (-5)
firstTrueChirho = firstChirho 0

-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
module ShowSupportChirho where
import GHC.Internal.Show (showSpace)
resultChirho = showParen True (showString "x") ""
unwrappedChirho = showParen False (showString "x") "!"
characterChirho = showChar 'q' "!"
spaceChirho = showSpace "!"

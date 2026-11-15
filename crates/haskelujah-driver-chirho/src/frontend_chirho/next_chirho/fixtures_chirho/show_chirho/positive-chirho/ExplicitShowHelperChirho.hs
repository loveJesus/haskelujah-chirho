-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
module ExplicitShowHelperChirho where
import GHC.Internal.Show (showSignedInt)
resultChirho = showSignedInt 11 (-5) "!"

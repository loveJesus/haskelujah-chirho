-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
module ShowGcChirho where
import GHC.Internal.Show (showSignedInt)
data NatChirho = ZeroChirho | SuccChirho NatChirho
doubleChirho ZeroChirho = ZeroChirho
doubleChirho (SuccChirho kChirho) = SuccChirho (SuccChirho (doubleChirho kChirho))
bigChirho = doubleChirho (doubleChirho (doubleChirho (doubleChirho (doubleChirho (doubleChirho (doubleChirho (doubleChirho (doubleChirho (doubleChirho (doubleChirho (doubleChirho (SuccChirho ZeroChirho))))))))))))
walkChirho ZeroChirho = showSignedInt 0 0 ""
walkChirho (SuccChirho kChirho) = showSignedInt 0 12 (walkChirho kChirho)
lengthChirho [] = 0
lengthChirho (_:restChirho) = 1 + lengthChirho restChirho
resultChirho = lengthChirho (walkChirho bigChirho)

-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE NoImplicitPrelude #-}
module NumShadowOpsChirho where
import GHC.Internal.Num (Num(..))
data TChirho = AChirho | BChirho
instance Num TChirho where
    fromInteger _ = BChirho
    xChirho + _ = xChirho
    _ * yChirho = yChirho
    abs xChirho = xChirho
    signum xChirho = xChirho
    negate AChirho = BChirho
    negate BChirho = AChirho
pickChirho AChirho = 'a'
pickChirho BChirho = 'b'
fromInteger _ = 'x'
negate _ = BChirho
resultChirho = pickChirho (-5)

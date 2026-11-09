-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, KindSignatures #-}
module CollideChirho where

import qualified RelayChirho as ForeignChirho

data TagChirho (aChirho :: kChirho) = TagChirho

class ShapeChirho (aChirho :: kChirho) where
  sidesChirho :: TagChirho aChirho -> Int

instance ShapeChirho Int where
  sidesChirho _ = 1

probeChirho :: Int
probeChirho = sidesChirho (TagChirho :: TagChirho Maybe)

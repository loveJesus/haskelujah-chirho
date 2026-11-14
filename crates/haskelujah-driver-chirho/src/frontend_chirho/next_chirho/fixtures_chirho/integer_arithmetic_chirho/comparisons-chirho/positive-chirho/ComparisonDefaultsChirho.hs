-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

module ComparisonDefaultsChirho where
data TChirho = AChirho | BChirho
instance Eq TChirho where
  AChirho == AChirho = True
  BChirho == BChirho = True
  _ == _ = False
instance Ord TChirho where
  compare AChirho AChirho = EQ
  compare BChirho BChirho = EQ
  compare AChirho BChirho = LT
  compare BChirho AChirho = GT
sameChirho :: Ord a => a -> a -> Bool
sameChirho xChirho yChirho = xChirho == yChirho
defaultEqChirho = if AChirho /= BChirho then 'd' else 'x'
defaultLessChirho = if AChirho < BChirho then 'l' else 'x'
defaultMaxChirho = case max AChirho BChirho of { BChirho -> 'm'; _ -> 'x' }
superclassChirho = if sameChirho 3 3 then 's' else 'x'
polymorphicChirho xChirho yChirho = xChirho < yChirho
homeChirho = if polymorphicChirho AChirho BChirho then 'h' else 'x'
integerChirho = if polymorphicChirho (3 :: Integer) 4 then 'i' else 'x'
bothChirho :: (Eq a, Eq b) => a -> a -> b -> b -> Bool
bothChirho aChirho aaChirho bChirho bbChirho = if aChirho == aaChirho then bChirho /= bbChirho else False
twoDictionariesChirho = if bothChirho AChirho AChirho (2 :: Integer) 3 then '2' else 'x'

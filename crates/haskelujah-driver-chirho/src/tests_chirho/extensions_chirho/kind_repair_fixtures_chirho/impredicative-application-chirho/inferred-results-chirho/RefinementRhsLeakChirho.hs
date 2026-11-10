-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

{-# LANGUAGE GADTs #-}
module RefinementRhsLeakChirho where
data GChirho aChirho where
  GIChirho :: GChirho Int
  GOChirho :: GChirho aChirho
leakChirho :: GChirho bChirho -> bChirho -> Int
leakChirho gChirho yChirho = let rChirho = case gChirho of { GOChirho -> yChirho; GIChirho -> yChirho } in rChirho

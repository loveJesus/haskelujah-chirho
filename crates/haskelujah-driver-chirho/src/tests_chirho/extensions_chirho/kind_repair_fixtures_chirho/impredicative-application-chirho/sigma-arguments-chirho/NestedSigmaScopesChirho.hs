-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes, TypeAbstractions, GADTs, TypeOperators #-}
module Main where
data PairChirho = PairChirho (forall aChirho bChirho. (Ord aChirho, Ord bChirho) => aChirho -> aChirho -> bChirho -> bChirho -> (Bool, Bool))
pairChirho :: PairChirho
pairChirho = PairChirho (\leftChirho rightChirho lowChirho highChirho -> (leftChirho < rightChirho, lowChirho < highChirho))
data StagedChirho = StagedChirho (forall aChirho. Ord aChirho => forall bChirho. Ord bChirho => aChirho -> bChirho -> (Bool, Bool))
stagedChirho :: StagedChirho
stagedChirho = StagedChirho (\ @aChirho @bChirho leftChirho rightChirho -> (leftChirho == leftChirho, rightChirho == rightChirho))
data EqualityChirho = EqualityChirho (forall aChirho. (aChirho ~ Int) => Ord aChirho => aChirho -> Bool)
equalityChirho :: EqualityChirho
equalityChirho = EqualityChirho (\valueChirho -> valueChirho == valueChirho)
main :: IO ()
main = do
  case pairChirho of
    PairChirho compareChirho -> print (compareChirho (-2.5 :: Double) 1.0 'a' 'b')
  case stagedChirho of
    StagedChirho sameChirho -> print (sameChirho (-2.5 :: Double) 'x')
  case equalityChirho of
    EqualityChirho sameIntChirho -> print (sameIntChirho 42)

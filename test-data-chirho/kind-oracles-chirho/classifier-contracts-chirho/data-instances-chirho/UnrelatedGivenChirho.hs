-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, TypeFamilies #-}
module UnrelatedGivenChirho where
data ProxyChirho aChirho = ProxyChirho
data EqualChirho aChirho bChirho where
  ReflChirho :: EqualChirho aChirho aChirho
data PairChirho aChirho bChirho = PairChirho aChirho bChirho
data RefinedChirho aChirho bChirho where
  RefinedChirho :: bChirho -> RefinedChirho Int bChirho
type family ResultChirho aChirho
badTupleChirho :: ProxyChirho cChirho -> EqualChirho aChirho bChirho -> ResultChirho cChirho -> ()
badTupleChirho _ proofChirho valueChirho = case (proofChirho, valueChirho) of
  (ReflChirho, True) -> ()
badPairChirho :: ProxyChirho cChirho -> PairChirho (EqualChirho aChirho bChirho) (ResultChirho cChirho) -> ()
badPairChirho _ pairChirho = case pairChirho of
  PairChirho ReflChirho True -> ()
badFieldChirho :: ProxyChirho cChirho -> RefinedChirho aChirho (ResultChirho cChirho) -> ()
badFieldChirho _ refinedChirho = case refinedChirho of
  RefinedChirho True -> ()

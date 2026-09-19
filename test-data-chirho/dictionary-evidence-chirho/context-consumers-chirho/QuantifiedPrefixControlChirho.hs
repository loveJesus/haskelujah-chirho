-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE QuantifiedConstraints, FlexibleInstances, UndecidableInstances #-}
module Main where
newtype FixChirho fChirho = FixChirho (fChirho (FixChirho fChirho))
data PairChirho aChirho = PairChirho aChirho aChirho
instance Show aChirho => Show (PairChirho aChirho) where
  show _ = error "pair show witness forced"
instance (forall bChirho. Show bChirho => Show (fChirho bChirho)) => Show (FixChirho fChirho) where
  show (FixChirho xChirho) = "F(" ++ show xChirho ++ ")"
main :: IO ()
main = putStrLn (show (FixChirho (PairChirho (FixChirho (PairChirho undefinedChirho undefinedChirho)) undefinedChirho)) `seq` "built")
  where undefinedChirho = error "not forced"

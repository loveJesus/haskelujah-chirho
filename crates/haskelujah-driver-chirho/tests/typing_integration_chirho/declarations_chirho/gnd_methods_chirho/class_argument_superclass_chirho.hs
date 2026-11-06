-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GeneralizedNewtypeDeriving, MultiParamTypeClasses, FlexibleInstances, FlexibleContexts #-}
module Main where

class Monad m => AskChirho r m

newtype ReaderChirho r a = ReaderChirho (r -> a)

runReaderChirho :: ReaderChirho r a -> r -> a
runReaderChirho (ReaderChirho f) = f

instance Functor (ReaderChirho r) where
  fmap f (ReaderChirho g) = ReaderChirho (\r -> f (g r))

instance Applicative (ReaderChirho r) where
  pure x = ReaderChirho (\_ -> x)
  ReaderChirho f <*> ReaderChirho g = ReaderChirho (\r -> f r (g r))

instance Monad (ReaderChirho r) where
  ReaderChirho g >>= k = ReaderChirho (\r -> runReaderChirho (k (g r)) r)

instance AskChirho r (ReaderChirho r)

newtype TChirho a x = TChirho (ReaderChirho a x)
  deriving (Functor, Applicative, Monad, AskChirho a)

sevenChirho :: AskChirho Int m => m Int
sevenChirho = return 3 >>= \n -> return (n + 4)

runTChirho :: TChirho a x -> a -> x
runTChirho (TChirho r) = runReaderChirho r

main :: IO ()
main = print (runTChirho (sevenChirho :: TChirho Int Int) 0)

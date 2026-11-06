-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

import ReaderBaseChirho

-- The Functor and Applicative instances this Monad's superclass slots need are another
-- module's.
instance Monad (ReaderChirho r) where
  ReaderChirho g >>= k = ReaderChirho (\r -> runReaderChirho (k (g r)) r)

sevenChirho :: Monad m => m Int
sevenChirho = return 3 >>= \n -> return (n + 4)

main :: IO ()
main = print (runReaderChirho (sevenChirho :: ReaderChirho Int Int) 0)

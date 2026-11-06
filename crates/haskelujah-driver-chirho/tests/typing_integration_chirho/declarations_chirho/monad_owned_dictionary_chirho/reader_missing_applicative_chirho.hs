-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

newtype ReaderChirho r a = ReaderChirho (r -> a)

runReaderChirho :: ReaderChirho r a -> r -> a
runReaderChirho (ReaderChirho f) = f

instance Functor (ReaderChirho r) where
  fmap f (ReaderChirho g) = ReaderChirho (\r -> f (g r))

instance Monad (ReaderChirho r) where
  ReaderChirho g >>= k = ReaderChirho (\r -> runReaderChirho (k (g r)) r)

sevenChirho :: Monad m => m Int
sevenChirho = return 3 >>= \n -> return (n + 4)

main :: IO ()
main = print (runReaderChirho (sevenChirho :: ReaderChirho Int Int) 0)

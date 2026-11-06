-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

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

askChirho :: ReaderChirho r r
askChirho = ReaderChirho id

scaledChirho :: Monad m => m Int -> m Int
scaledChirho actionChirho = do
  baseChirho <- actionChirho
  doubledChirho <- fmap (* 2) (pure baseChirho)
  return (doubledChirho + 1)

main :: IO ()
main = do
  print (runReaderChirho (scaledChirho askChirho) 20)
  print (runReaderChirho (scaledChirho (fmap (+ 1) askChirho)) 4)
  scaledChirho (return 3) >>= print

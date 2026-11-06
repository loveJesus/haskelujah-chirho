-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

newtype StateChirho s a = StateChirho (s -> (a, s))

runStateChirho :: StateChirho s a -> s -> (a, s)
runStateChirho (StateChirho f) = f

firstChirho :: (a -> b) -> (a, s) -> (b, s)
firstChirho f (a, s1) = (f a, s1)

applyChirho :: (a -> b, s) -> (s -> (a, s)) -> (b, s)
applyChirho (h, s1) g = firstChirho h (g s1)

continueChirho :: (a -> StateChirho s b) -> (a, s) -> (b, s)
continueChirho k (a, s1) = runStateChirho (k a) s1

instance Functor (StateChirho s) where
  fmap f (StateChirho g) = StateChirho (\s -> firstChirho f (g s))

instance Applicative (StateChirho s) where
  pure a = StateChirho (\s -> (a, s))
  StateChirho f <*> StateChirho g = StateChirho (\s -> applyChirho (f s) g)

-- The instance writes `>>=` alone, so `>>` is the class's own default.
instance Monad (StateChirho s) where
  StateChirho g >>= k = StateChirho (\s -> continueChirho k (g s))

logChirho :: Int -> StateChirho [Int] ()
logChirho n = StateChirho (\s -> ((), s ++ [n]))

-- `>>` through the Monad dictionary the caller passes.
aroundChirho :: Monad m => m () -> m () -> m ()
aroundChirho outer inner = outer >> inner >> outer

-- Statements without a bind sequence the same way.
sixSevenChirho :: StateChirho [Int] ()
sixSevenChirho = do
  logChirho 6
  logChirho 7

main :: IO ()
main = do
  print (snd (runStateChirho (logChirho 1 >> logChirho 2) []))
  print (snd (runStateChirho (aroundChirho (logChirho 3) (logChirho 4)) []))
  print (fst (runStateChirho (logChirho 5 >> pure 'z') []))
  print (snd (runStateChirho sixSevenChirho []))

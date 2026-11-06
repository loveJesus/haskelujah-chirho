-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

newtype TallyChirho t a = TallyChirho (Int -> (a, Int))

runTallyChirho :: TallyChirho t a -> Int -> (a, Int)
runTallyChirho (TallyChirho f) = f

instance Functor (TallyChirho t) where
  fmap f (TallyChirho g) = TallyChirho (\n -> case g n of (a, m) -> (f a, m))

instance Applicative (TallyChirho t) where
  pure a = TallyChirho (\n -> (a, n))
  TallyChirho f <*> TallyChirho g =
    TallyChirho (\n -> case f n of (h, m) -> case g m of (a, k) -> (h a, k))

-- The instance writes `>>=` alone, so `>>` is the class's own default.
instance Monad (TallyChirho t) where
  TallyChirho g >>= k = TallyChirho (\n -> case g n of (a, m) -> runTallyChirho (k a) m)

tickChirho :: TallyChirho Bool ()
tickChirho = TallyChirho (\n -> ((), n + 1))

main :: IO ()
main = do
  -- `>>` at a concrete head this module declares.
  print (snd (runTallyChirho (tickChirho >> tickChirho >> tickChirho) 0))
  -- The program's own values spelled like the methods keep their own bindings.
  print (fmap 2, pure 5, return 10)
  where
    fmap :: Int -> Int
    fmap n = n * 100
    pure :: Int -> Int
    pure n = n + 1000
    return :: Int -> Int
    return n = n - 1

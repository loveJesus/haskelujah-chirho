-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16

-- | Built-in pseudo-random number generation. No package install needed.
-- Uses a simple linear congruential generator.
module Haskelujah.Random (
    Gen,
    mkGen,
    nextInt,
    nextDouble,
    randomList,
    shuffle,
    choice,
) where

-- | A pseudo-random number generator state.
newtype Gen = Gen Int

-- | Create a generator from a seed.
mkGen :: Int -> Gen
mkGen = Gen

-- | Generate the next random Int and new generator.
nextInt :: Gen -> (Int, Gen)
nextInt (Gen s) =
    let s' = (s * 1103515245 + 12345) `mod` 2147483648
    in (s', Gen s')

-- | Generate a random Double in [0, 1).
nextDouble :: Gen -> (Double, Gen)
nextDouble g =
    let (n, g') = nextInt g
        d = fromIntegral (abs n) / 2147483648.0
    in (d, g')

-- | Generate a list of n random Ints.
randomList :: Int -> Gen -> ([Int], Gen)
randomList 0 g = ([], g)
randomList n g =
    let (x, g') = nextInt g
        (xs, g'') = randomList (n - 1) g'
    in (x : xs, g'')

-- | Choose a random element from a list.
choice :: [a] -> Gen -> (a, Gen)
choice [] _ = error "choice: empty list"
choice xs g =
    let (n, g') = nextInt g
        idx = abs n `mod` length xs
    in (indexAt idx xs, g')

-- | Shuffle a list (Fisher-Yates, functional style).
shuffle :: [a] -> Gen -> ([a], Gen)
shuffle [] g = ([], g)
shuffle [x] g = ([x], g)
shuffle xs g =
    let (n, g') = nextInt g
        idx = abs n `mod` length xs
        (picked, rest) = removeAt idx xs
        (shuffled, g'') = shuffle rest g'
    in (picked : shuffled, g'')
  where
    removeAt 0 (y:ys) = (y, ys)
    removeAt i (y:ys) = let (p, rs) = removeAt (i-1) ys in (p, y:rs)
    removeAt _ [] = error "removeAt"

indexAt :: Int -> [a] -> a
indexAt 0 (x:_) = x
indexAt n (_:xs) = indexAt (n-1) xs
indexAt _ [] = error "indexAt: index out of range"

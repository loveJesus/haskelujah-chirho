-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE Haskell2010, ExplicitForAll #-}
fChirho :: forall a_chirho. [a_chirho] -> [a_chirho]
fChirho xs_chirho = ys_chirho where { ys_chirho :: [a_chirho]; ys_chirho = xs_chirho }
main = putStrLn "ok"

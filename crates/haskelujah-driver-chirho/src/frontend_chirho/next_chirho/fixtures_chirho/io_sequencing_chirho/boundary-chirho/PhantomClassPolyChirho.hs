-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GHC2021 #-}
data PChirho a_chirho = PChirho
data TChirho a_chirho = TChirho a_chirho
class CChirho f_chirho where
  mChirho :: PChirho f_chirho -> Integer
instance CChirho TChirho where
  mChirho _ = 0
main = putStrLn "ok"

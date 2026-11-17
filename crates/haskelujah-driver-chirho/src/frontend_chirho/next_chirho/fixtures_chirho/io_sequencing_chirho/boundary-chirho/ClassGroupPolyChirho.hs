-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GHC2021 #-}
class AChirho f_chirho where
  aChirho :: f_chirho x_chirho -> Integer
class AChirho g_chirho => BChirho g_chirho where
  bChirho :: g_chirho IO -> Integer
data TChirho x_chirho = TChirho x_chirho
instance AChirho TChirho where
  aChirho _ = 0
main = putStrLn "ok"

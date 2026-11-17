-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
{-# LANGUAGE Haskell2010 #-}
data BoxChirho a_chirho = BoxChirho a_chirho
class HomeMapChirho f_chirho where
  homeMapChirho :: (a_chirho -> b_chirho) -> f_chirho a_chirho -> f_chirho b_chirho
instance HomeMapChirho BoxChirho where
  homeMapChirho function_chirho (BoxChirho value_chirho) = BoxChirho (function_chirho value_chirho)
resultChirho :: Integer
resultChirho = case homeMapChirho (\value_chirho -> value_chirho + 1) (BoxChirho 2) of { BoxChirho value_chirho -> value_chirho }
main = print resultChirho

-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE Haskell2010 #-}
class BoxChirho f_chirho where
  unboxChirho :: f_chirho IO -> Integer
applyChirho :: t_chirho a_chirho -> t_chirho a_chirho
applyChirho x_chirho = x_chirho
useChirho :: BoxChirho f_chirho => f_chirho IO -> Integer
useChirho v_chirho = unboxChirho (applyChirho v_chirho)
main = putStrLn "ok"

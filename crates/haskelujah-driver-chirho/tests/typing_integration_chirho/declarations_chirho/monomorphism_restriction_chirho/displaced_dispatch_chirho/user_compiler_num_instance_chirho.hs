-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

-- An instance of the compiler's own Num: its literals and its arithmetic are the program's.
newtype MeterChirho = MeterChirho Int

instance Show MeterChirho where
  show (MeterChirho valueChirho) = show valueChirho ++ "m"

instance Num MeterChirho where
  MeterChirho leftChirho + MeterChirho rightChirho = MeterChirho (leftChirho + rightChirho)
  MeterChirho leftChirho * MeterChirho rightChirho = MeterChirho (leftChirho * rightChirho)
  abs (MeterChirho valueChirho) = MeterChirho (abs valueChirho)
  signum (MeterChirho valueChirho) = MeterChirho (signum valueChirho)
  fromInteger valueChirho = MeterChirho (fromInteger valueChirho * 100)
  negate (MeterChirho valueChirho) = MeterChirho (negate valueChirho)

main :: IO ()
main = print (2 + 3 :: MeterChirho)

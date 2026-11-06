-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

sevenChirho :: Monad m => m Int
sevenChirho = return 3 >>= \n -> return (n + 4)

main :: IO ()
main = sevenChirho >>= print

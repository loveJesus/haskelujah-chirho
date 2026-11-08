-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

-- The lambda-valued member has a signature, so the group is unrestricted.
evenChirho countChirho = countChirho == 0 || oddChirho (countChirho - 1)

oddChirho :: (Eq a, Num a) => a -> Bool
oddChirho = \countChirho -> countChirho /= 0 && evenChirho (countChirho - 1)

main :: IO ()
main = print (evenChirho (4 :: Int), evenChirho (3 :: Integer))

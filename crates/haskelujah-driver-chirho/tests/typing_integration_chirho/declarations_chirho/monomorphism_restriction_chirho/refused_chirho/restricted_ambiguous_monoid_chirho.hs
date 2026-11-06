-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

-- Restricted, never fixed by a use, and Monoid has no default: the type is ambiguous.
emptyChirho = mempty

main :: IO ()
main = emptyChirho `seq` return ()

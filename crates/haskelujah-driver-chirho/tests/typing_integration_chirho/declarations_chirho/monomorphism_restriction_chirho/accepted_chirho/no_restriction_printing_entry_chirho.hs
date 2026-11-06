-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE NoMonomorphismRestriction #-}
module Main where

-- Generalized over its Num result; the entry's role instantiates it, so its action runs.
main = print 7 >> return 7

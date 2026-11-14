-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

{-# LANGUAGE NoImplicitPrelude #-}
module HomeOrderingChirho where
import GHC.Internal.Num (Num(..))
import GHC.Internal.Bignum.Integer (Integer)
import GHC.Internal.Classes (Ord(..))
data Ordering = LT | EQ | GT
resultChirho = case compare (1 :: Integer) 2 of { LT -> 'l'; EQ -> 'e'; GT -> 'g' }

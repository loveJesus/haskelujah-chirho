-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

module ComparisonPartialChirho where
import GHC.Internal.Bignum.Integer (Integer, integerCompare, integerEq)
boomChirho :: Integer
boomChirho = partialChirho True
partialChirho valueChirho = case valueChirho of { False -> 0 }
holdChirho _ = 'p'
lazyChirho = holdChirho (integerCompare boomChirho)
applyChirho fChirho = fChirho 42
resultChirho = case applyChirho (integerCompare 41) of { LT -> 'l'; _ -> 'x' }
equalChirho = if applyChirho (integerEq 42) then 't' else 'x'

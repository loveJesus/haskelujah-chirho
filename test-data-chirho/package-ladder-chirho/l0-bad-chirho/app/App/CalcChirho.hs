-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

module App.CalcChirho (labelChirho, summaryChirho) where

labelChirho :: String
labelChirho = "calc"

-- A type error GHC rejects: a String where an Int belongs.
summaryChirho :: [Int] -> [(String, Int)]
summaryChirho valuesChirho = [("count", "eight"), ("total", sum valuesChirho)]

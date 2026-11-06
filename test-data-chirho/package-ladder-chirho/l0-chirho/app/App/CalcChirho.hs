-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

module App.CalcChirho (labelChirho, summaryChirho) where

labelChirho :: String
labelChirho = "calc"

summaryChirho :: [Int] -> [(String, Int)]
summaryChirho valuesChirho =
  [ ("count", length valuesChirho)
  , ("total", sum valuesChirho)
  , ("max", maximum valuesChirho)
  ]

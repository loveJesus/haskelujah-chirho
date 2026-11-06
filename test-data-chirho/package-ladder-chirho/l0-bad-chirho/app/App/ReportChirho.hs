-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

module App.ReportChirho (labelChirho, renderLinesChirho) where

labelChirho :: String
labelChirho = "report"

renderLinesChirho :: [(String, Int)] -> [String]
renderLinesChirho = map renderLineChirho
  where
    renderLineChirho (nameChirho, valueChirho) = nameChirho ++ " = " ++ show valueChirho

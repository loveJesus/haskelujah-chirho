-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

module Main (main) where

import qualified App.CalcChirho as CalcChirho
import qualified App.ReportChirho as ReportChirho

-- Both helper modules export a binding named labelChirho; each qualified use must reach its own.
main :: IO ()
main = do
  let valuesChirho = [3, 1, 4, 1, 5, 9, 2, 6] :: [Int]
  putStrLn ReportChirho.labelChirho
  putStrLn CalcChirho.labelChirho
  mapM_ putStrLn (ReportChirho.renderLinesChirho (CalcChirho.summaryChirho valuesChirho))

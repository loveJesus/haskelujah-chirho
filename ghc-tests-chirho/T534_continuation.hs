-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
-- TEST: compile_and_run
-- EXPECTED: 120\n55\n[5,4,3,2,1]
module Main where
-- Continuation-passing style — tests closures deeply
factCPS :: Int -> (Int -> a) -> a
factCPS 0 k = k 1
factCPS n k = factCPS (n-1) (\r -> k (n * r))
fibCPS :: Int -> (Int -> a) -> a
fibCPS 0 k = k 0
fibCPS 1 k = k 1
fibCPS n k = fibCPS (n-1) (\a -> fibCPS (n-2) (\b -> k (a + b)))
-- Build a list via CPS
buildListCPS :: Int -> ([Int] -> a) -> a
buildListCPS 0 k = k []
buildListCPS n k = buildListCPS (n-1) (\rest -> k (n : rest))
main :: IO ()
main = do
  print (factCPS 5 id)
  print (fibCPS 10 id)
  print (buildListCPS 5 id)

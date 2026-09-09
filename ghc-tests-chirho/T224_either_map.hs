-- TEST: compile_and_run
-- EXPECTED: 43\n-1
module Main where
import Prelude hiding (Either(..))
data Either a b = Left a | Right b
mapRight :: (Int -> Int) -> Either Int Int -> Either Int Int
mapRight _ (Left x) = Left x
mapRight f (Right x) = Right (f x)
fromRight def (Left _) = def; fromRight _ (Right x) = x
main = do
  print (fromRight (-1) (mapRight (+1) (Right 42)))
  print (fromRight (-1) (mapRight (+1) (Left 99)))

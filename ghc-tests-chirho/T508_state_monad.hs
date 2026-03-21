-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
-- TEST: compile_and_run
-- EXPECTED: (42,3)\n(5,hello)
module Main where

-- Mini state monad using explicit state passing and case-matching
type State s a = s -> (a, s)

bindState :: State s a -> (a -> State s b) -> State s b
bindState m f s = case m s of
  (a, s2) -> f a s2

returnState :: a -> State s a
returnState a s = (a, s)

getState :: State s s
getState s = (s, s)

incr :: State Int ()
incr s = ((), s + 1)

counter :: State Int Int
counter = bindState incr (\_ ->
          bindState incr (\_ ->
          bindState incr (\_ ->
          bindState getState (\n ->
          returnState (n * 14)))))

appendStr :: String -> State String ()
appendStr extra s = ((), s ++ extra)

strBuilder :: State String Int
strBuilder = bindState (appendStr "hel") (\_ ->
             bindState (appendStr "lo") (\_ ->
             returnState 5))

showPairInt :: (Int, Int) -> String
showPairInt (a, b) = "(" ++ show a ++ "," ++ show b ++ ")"

showPairIntStr :: (Int, String) -> String
showPairIntStr (a, b) = "(" ++ show a ++ "," ++ b ++ ")"

main :: IO ()
main = do
  putStrLn (showPairInt (counter 0))
  putStrLn (showPairIntStr (strBuilder ""))

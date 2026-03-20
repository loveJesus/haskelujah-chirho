-- TEST: compile_and_run
-- EXPECTED: 150\n2\n2
module Main where
data GameState = GameState Int Int Int
score :: GameState -> Int
score (GameState s _ _) = s
lives :: GameState -> Int
lives (GameState _ l _) = l
level :: GameState -> Int
level (GameState _ _ lv) = lv
addScore :: Int -> GameState -> GameState
addScore pts (GameState s l lv) = GameState (s + pts) l lv
loseLife :: GameState -> GameState
loseLife (GameState s l lv) = GameState s (l - 1) lv
nextLevel :: GameState -> GameState
nextLevel (GameState s l lv) = GameState s l (lv + 1)
playTurn :: GameState -> Int -> GameState
playTurn gs pts
  | pts > 0 = addScore pts gs
  | pts == 0 = loseLife gs
  | otherwise = nextLevel gs
main :: IO ()
main = do
  let g = playTurn (playTurn (playTurn (playTurn (GameState 0 3 1) 100) 50) 0) (-1)
  print (score g)
  print (lives g)
  print (level g)

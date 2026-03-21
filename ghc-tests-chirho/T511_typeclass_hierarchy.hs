-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
-- TEST: compile_and_run
-- EXPECTED: Pt(1,2)\nPt(3,4)\nPt(4,6)\n5
module Main where
data Point = Pt Int Int

showPoint :: Point -> String
showPoint (Pt x y) = "Pt(" ++ show x ++ "," ++ show y ++ ")"

addPoints :: Point -> Point -> Point
addPoints (Pt x1 y1) (Pt x2 y2) = Pt (x1 + x2) (y1 + y2)

dist :: Point -> Int
dist (Pt x y) = abs x + abs y

main = do
  let p1 = Pt 1 2
  let p2 = Pt 3 4
  putStrLn (showPoint p1)
  putStrLn (showPoint p2)
  putStrLn (showPoint (addPoints p1 p2))
  print (dist (Pt 2 3))

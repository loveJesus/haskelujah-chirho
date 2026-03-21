-- TEST: compile_and_run
-- EXPECTED: 45
module Main where
mySum [] = 0; mySum (x:xs) = x + mySum xs
qsort [] = []; qsort (x:xs) = append (qsort lo) (x : qsort hi) where lo = f (<=x) xs; hi = f (>x) xs
f _ [] = []; f p (x:xs) = if p x then x : f p xs else f p xs
append [] ys = ys; append (x:xs) ys = x : append xs ys
main = print (mySum (qsort [5,3,8,1,9,2,7,4,6]))

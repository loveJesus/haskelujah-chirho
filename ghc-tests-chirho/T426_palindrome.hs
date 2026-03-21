-- TEST: compile_and_run
-- EXPECTED: 1\n0
module Main where
bti True = 1; bti False = 0
myReverse xs = go xs [] where go [] a = a; go (x:rest) a = go rest (x:a)
isPalindrome xs = eq xs (myReverse xs)
eq [] [] = True; eq (x:xs) (y:ys) = if x == y then eq xs ys else False; eq _ _ = False
main = do { print (bti (isPalindrome [1,2,3,2,1])); print (bti (isPalindrome [1,2,3])) }

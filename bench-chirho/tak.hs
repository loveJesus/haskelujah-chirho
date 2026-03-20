-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
module Main where
tak :: Int -> Int -> Int -> Int
tak x y z = if y >= x then z
            else tak (tak (x-1) y z) (tak (y-1) z x) (tak (z-1) x y)
main :: IO ()
main = print (tak 30 20 10)

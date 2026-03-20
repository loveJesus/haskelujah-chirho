-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
module Main where
ack :: Int -> Int -> Int
ack 0 n = n + 1
ack m 0 = ack (m - 1) 1
ack m n = ack (m - 1) (ack m (n - 1))
main :: IO ()
main = print (ack 3 10)

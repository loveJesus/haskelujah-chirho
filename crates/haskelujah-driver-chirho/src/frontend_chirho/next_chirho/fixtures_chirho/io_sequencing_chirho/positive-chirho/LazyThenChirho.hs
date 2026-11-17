-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
missChirho :: Bool -> a_chirho
missChirho True = missChirho True
boomChirho :: a_chirho
boomChirho = missChirho False
main = return (boomChirho :: Integer) >> putStrLn "ok"

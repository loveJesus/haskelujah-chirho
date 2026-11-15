-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
module ShowDemandChirho where
boomPChirho :: Int
boomPChirho = case False of True -> 0
boomIntChirho :: Int
boomIntChirho = case True of False -> 0
boomIntegerChirho :: Integer
boomIntegerChirho = case True of False -> 0
intOrderChirho = showsPrec boomPChirho boomIntChirho ""
integerOrderChirho = showsPrec boomPChirho boomIntegerChirho ""
positivePChirho = showsPrec boomPChirho (5 :: Integer) ""
tailChirho = shows (12 :: Integer) (case True of False -> "")

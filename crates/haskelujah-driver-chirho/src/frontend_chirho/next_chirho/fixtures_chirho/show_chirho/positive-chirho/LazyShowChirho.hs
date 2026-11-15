-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
module LazyShowChirho where
firstTwoChirho (aChirho:bChirho:_) = [aChirho,bChirho]
firstFourChirho (aChirho:bChirho:cChirho:dChirho:_) = [aChirho,bChirho,cChirho,dChirho]
boomChirho = case True of False -> ""
intChirho = firstTwoChirho (shows (12 :: Int) boomChirho)
intParenChirho = firstFourChirho (showsPrec 7 (-5 :: Int) boomChirho)
integerChirho = firstTwoChirho (shows (1180591620717411303424 :: Integer) boomChirho)
integerParenChirho = firstFourChirho (showsPrec 7 (-1180591620717411303424 :: Integer) boomChirho)
stringChirho = firstTwoChirho (showString "ab" boomChirho)
partialIntChirho = showsPrec (case False of True -> 0) (1 :: Int)
partialIntegerChirho = showsPrec (case False of True -> 0) (1 :: Integer)

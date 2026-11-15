-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
module AppendSourceChirho where
firstChirho (xChirho:_) = xChirho
secondChirho (_:yChirho:_) = yChirho
fourthChirho (_:_:_:xChirho:_) = xChirho
boomChirho = case True of False -> []
resultChirho = firstChirho ("ab" ++ "cd")
secondResultChirho = secondChirho ("a" ++ "bc")
lazyFirstChirho = firstChirho ("ab" ++ boomChirho)
lazySecondChirho = secondChirho ("ab" ++ boomChirho)
emptyLeftChirho = firstChirho ([] ++ ['q'])
chainChirho = fourthChirho (1 : 2 : [3] ++ [4])
partialChirho = (++) boomChirho

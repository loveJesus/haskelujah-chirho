-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
module AppendDemandChirho where
firstChirho (xChirho:_) = xChirho
boomChirho = case True of False -> []
leftChirho = firstChirho (boomChirho ++ ['q'])
tailChirho = firstChirho ([] ++ boomChirho)

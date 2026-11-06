-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TransformListComp #-}
module Main where
groupRowsChirho :: Eq bChirho => (aChirho -> bChirho) -> [aChirho] -> [[aChirho]]
groupRowsChirho keyChirho [] = []
groupRowsChirho keyChirho (firstChirho:restChirho) = (firstChirho : filter (\elementChirho -> keyChirho elementChirho == keyChirho firstChirho) restChirho) : groupRowsChirho keyChirho (filter (\elementChirho -> keyChirho elementChirho /= keyChirho firstChirho) restChirho)
valueChirho :: [([Int],[Int])]
valueChirho = [ (xChirho,yChirho) | (xChirho,yChirho) <- ([(1,2),(1,3),(2,4)] :: [(Int,Int)]), then group by xChirho using groupRowsChirho ]
main :: IO ()
main = print valueChirho

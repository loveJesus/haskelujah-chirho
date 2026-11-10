-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

{-# LANGUAGE GADTs #-}
module RefinementEscapeChirho where
data WChirho bChirho where
  MkWChirho :: eChirho -> WChirho (Maybe eChirho)
escChirho :: WChirho bChirho -> bChirho -> bChirho
escChirho wChirho yChirho = let rChirho = case wChirho of MkWChirho _ -> yChirho in rChirho

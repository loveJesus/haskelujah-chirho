-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

-- A function binding generalizes: (Monad m, Num b) => (t -> m a) -> [t] -> m b.
mapMChirho_ actionChirho valuesChirho = case valuesChirho of
  [] -> return 0
  (headChirho : restChirho) -> actionChirho headChirho >> mapMChirho_ actionChirho restChirho

-- An unsigned entry is restricted: its Num result is defaulted to Integer, never abstracted.
main = mapMChirho_ putStrLn ["hello", "world"]

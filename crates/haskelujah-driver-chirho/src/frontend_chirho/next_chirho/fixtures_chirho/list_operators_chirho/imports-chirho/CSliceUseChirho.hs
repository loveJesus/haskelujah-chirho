-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
module CSliceUseChirho where
import ASliceFacadeChirho
pickChirho bChirho _ = case bChirho of { True -> 't'; False -> 'f' }
resultChirho = 1 `geInt` 2 `pickChirho` 3

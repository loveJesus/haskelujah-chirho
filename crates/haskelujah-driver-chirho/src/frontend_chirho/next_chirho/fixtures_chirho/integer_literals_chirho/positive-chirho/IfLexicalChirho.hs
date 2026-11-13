-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RebindableSyntax #-}
module IfLexicalChirho where
data FlagChirho = ReadyChirho | StopChirho
fromInteger ignoredChirho = ReadyChirho
ifThenElse ReadyChirho yesChirho noChirho = yesChirho
ifThenElse StopChirho yesChirho noChirho = noChirho
resultChirho = if 0 then 'i' else '!'

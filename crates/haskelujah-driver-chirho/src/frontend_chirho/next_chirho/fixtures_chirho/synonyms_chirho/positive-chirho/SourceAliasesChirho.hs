-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
{-# LANGUAGE NoImplicitPrelude #-}
module SourceAliasesChirho where
import GHC.Internal.Base (String)
import GHC.Internal.Show (ShowS)
renderChirho :: ShowS
renderChirho tailChirho = tailChirho
resultChirho :: String
resultChirho = renderChirho "source aliases"

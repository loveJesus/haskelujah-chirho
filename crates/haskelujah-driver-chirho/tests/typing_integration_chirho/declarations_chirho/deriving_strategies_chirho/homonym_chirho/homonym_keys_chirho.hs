-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DerivingStrategies, GeneralizedNewtypeDeriving #-}
module KeysChirho (KeyChirho (..), showKeyChirho) where
newtype KeyChirho = KeyChirho Int deriving newtype Show
showKeyChirho :: Int -> String
showKeyChirho valueChirho = show (KeyChirho valueChirho)

-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, TypeApplications, DataKinds #-}
module AutoKindChirho where
data ProxyChirho valueChirho = ProxyChirho
autoChirho :: ProxyChirho @Bool True -> ()
autoChirho _ = ()

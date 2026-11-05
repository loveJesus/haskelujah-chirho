-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE FlexibleContexts, PolyKinds #-}
module PolyClassColdChirho where
import qualified SupplierChirho as SupplierChirho
import Data.Proxy (Proxy)
valueChirho :: SupplierChirho.CChirho Maybe => Proxy Maybe
valueChirho = SupplierChirho.proxyChirho

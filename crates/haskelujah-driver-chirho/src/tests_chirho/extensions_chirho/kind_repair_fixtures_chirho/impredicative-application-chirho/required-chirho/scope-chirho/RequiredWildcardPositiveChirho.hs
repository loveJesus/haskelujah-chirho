-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RequiredTypeArguments, TypeAbstractions, KindSignatures, PolyKinds, DataKinds, ScopedTypeVariables #-}
module RequiredWildcardPositiveChirho where
import Data.Kind (Type)
import Data.Proxy
produceChirho :: forall (aChirho :: Type) -> Proxy aChirho
produceChirho _ = Proxy
valueChirho :: Proxy Int
valueChirho = produceChirho _

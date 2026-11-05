-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GADTs, RankNTypes, PolyKinds, TypeApplications, StandaloneKindSignatures #-}
module AliasResultPrefixPositiveChirho where
import Data.Kind (Type)
import Data.Proxy (Proxy(..))
type ResultChirho :: Type -> forall kChirho. kChirho -> Type
type ResultChirho ignoredChirho = (Proxy :: forall kChirho. kChirho -> Type)
valueChirho :: ResultChirho Bool @Type Int
valueChirho = Proxy

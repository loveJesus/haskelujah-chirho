-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, DerivingStrategies, FlexibleInstances, GeneralizedNewtypeDeriving, KindSignatures, MultiParamTypeClasses, TypeOperators, NoStarIsType #-}
module LocalUniverseShadowChirho where
import qualified Data.Kind as KindChirho
data Type = TypeChirho
class MarkerChirho rChirho (fChirho :: Type -> Type)
instance MarkerChirho () Maybe
newtype WrapChirho aChirho = WrapChirho (Maybe aChirho) deriving newtype (MarkerChirho ())

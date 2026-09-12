-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, TypeFamilies, DataKinds #-}
module ListKindFamilyRhsChirho where
import Data.Kind (Type)
data BoxChirho (funChirho :: [kindChirho] -> kindChirho -> Type) (argChirho :: kindChirho)
type family ResultChirho (funChirho :: [kindChirho] -> kindChirho -> Type) (argChirho :: kindChirho) where
  ResultChirho funChirho argChirho = BoxChirho funChirho argChirho

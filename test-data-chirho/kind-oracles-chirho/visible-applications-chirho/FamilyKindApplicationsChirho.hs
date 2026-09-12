-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, GADTs, TypeApplications, TypeFamilies, RankNTypes #-}
module FamilyKindApplicationsChirho where
import Data.Kind (Type)
type family ClassifierChirho indexChirho where
  ClassifierChirho Type = Type -> Type
  ClassifierChirho (Type -> Type) = Type
data BoxChirho :: ClassifierChirho indexChirho -> Type
selectChirho :: BoxChirho @Type Maybe -> BoxChirho @(Type -> Type) Int -> ()
selectChirho _ _ = ()

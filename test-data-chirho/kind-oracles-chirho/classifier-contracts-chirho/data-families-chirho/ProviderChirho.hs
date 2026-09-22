-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies #-}
module ProviderChirho where
data family DataChirho aChirho
type family TypeChirho aChirho
class ClassChirho aChirho where
  data AssociatedDataChirho aChirho
  type AssociatedTypeChirho aChirho

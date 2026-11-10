-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE ExistentialQuantification, ScopedTypeVariables, FlexibleInstances, NoPolyKinds #-}
module InstancePatternRigidChirho where
data TChirho aChirho = forall bChirho. MkTChirho bChirho
class ClassChirho tChirho where methodChirho :: tChirho -> ()
instance ClassChirho (TChirho aChirho) where
  methodChirho (MkTChirho (_ :: aChirho)) = ()

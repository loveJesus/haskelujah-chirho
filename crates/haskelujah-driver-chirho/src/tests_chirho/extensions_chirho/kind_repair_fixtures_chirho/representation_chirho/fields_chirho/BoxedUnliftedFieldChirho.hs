-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, DataKinds, KindSignatures #-}
module BoxedUnliftedFieldChirho where
import GHC.Exts (TYPE, RuntimeRep(..), Levity(..))
data BoxChirho (aChirho :: TYPE (BoxedRep 'Unlifted)) = MkChirho aChirho

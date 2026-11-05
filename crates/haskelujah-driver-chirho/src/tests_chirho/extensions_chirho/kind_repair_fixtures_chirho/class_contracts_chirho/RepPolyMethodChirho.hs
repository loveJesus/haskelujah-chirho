-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE KindSignatures, PolyKinds #-}
module RepPolyMethodChirho where

import GHC.Exts (TYPE)

-- A class method signature is checked at Type; this method's type is TYPE r with r rigid.
class PickChirho (a :: TYPE r) where
  pickChirho :: a

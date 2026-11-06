-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE FlexibleInstances, PolyKinds, MagicHash, KindSignatures #-}
module ExplicitPolykindedInstanceChirho where
import GHC.Exts (TYPE, Int#)
import Data.Proxy (Proxy(..))
class TagChirho (aChirho :: TYPE rChirho) where
  tagChirho :: Proxy aChirho -> ()
instance TagChirho (aChirho :: TYPE rChirho) where
  tagChirho _ = ()
unliftedChirho :: Proxy Int# -> ()
unliftedChirho = tagChirho

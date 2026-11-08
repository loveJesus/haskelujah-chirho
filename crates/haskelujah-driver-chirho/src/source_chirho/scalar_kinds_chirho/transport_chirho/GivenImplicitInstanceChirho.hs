-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE FlexibleInstances, FlexibleContexts, PolyKinds, MagicHash, KindSignatures #-}
module GivenImplicitInstanceChirho where
import GHC.Exts (TYPE, Int#)
import Data.Proxy (Proxy(..))
class TagChirho (aChirho :: TYPE rChirho) where
  tagChirho :: Proxy aChirho -> ()
instance TagChirho aChirho where
  tagChirho _ = ()
unliftedChirho :: TagChirho Int# => Proxy Int# -> ()
unliftedChirho = tagChirho

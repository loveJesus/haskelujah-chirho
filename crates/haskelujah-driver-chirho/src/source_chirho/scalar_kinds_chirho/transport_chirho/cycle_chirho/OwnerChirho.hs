-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, FlexibleContexts, FlexibleInstances, KindSignatures, MagicHash, PolyKinds #-}
module OwnerChirho where
import GHC.Exts (TYPE, Int#)
import Data.Proxy (Proxy)
import PeerChirho ()
class TagChirho (aChirho :: TYPE rChirho) where
  tagChirho :: Proxy aChirho -> ()
instance TagChirho aChirho where
  tagChirho _ = ()
valueChirho :: Proxy Int# -> ()
valueChirho = tagChirho

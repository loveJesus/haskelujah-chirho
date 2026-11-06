-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, FlexibleInstances, MagicHash #-}
module PeerChirho where
import {-# SOURCE #-} OwnerChirho (TagChirho(..))
import GHC.Exts (Int#)
instance TagChirho Int# where
  tagChirho _ = ()

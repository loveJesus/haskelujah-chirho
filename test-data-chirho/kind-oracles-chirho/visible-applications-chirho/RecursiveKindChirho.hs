-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, PolyKinds, KindSignatures, TypeApplications #-}
module Main where

data TreeChirho (valueChirho :: kindChirho)
  = LeafChirho Int | BranchChirho (TreeChirho valueChirho)

data EvenChirho (valueChirho :: leftKindChirho)
  = EndChirho Int | NextChirho (OddChirho valueChirho)
data OddChirho (valueChirho :: rightKindChirho)
  = MoreChirho (EvenChirho valueChirho)

peelChirho :: TreeChirho @Bool True -> Int
peelChirho (LeafChirho numberChirho) = numberChirho
peelChirho (BranchChirho restChirho) = peelChirho restChirho

evenChirho :: EvenChirho @Bool False -> Int
evenChirho (EndChirho numberChirho) = numberChirho
evenChirho (NextChirho (MoreChirho restChirho)) = evenChirho restChirho

main = do
  print (peelChirho (BranchChirho (LeafChirho 42)))
  print (evenChirho (NextChirho (MoreChirho (EndChirho 7))))

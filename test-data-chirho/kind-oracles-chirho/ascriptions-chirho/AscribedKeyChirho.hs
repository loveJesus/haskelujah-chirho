-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, GADTs, KindSignatures, PolyKinds, ScopedTypeVariables, TypeFamilies #-}
module Main where
import Data.Kind (Type)

data ProxyChirho (valueChirho :: Type) = ProxyChirho
data HiddenChirho where
  PackChirho :: forall keyChirho. ProxyChirho keyChirho -> HiddenChirho

type family KeyChirho (packedChirho :: HiddenChirho) :: Type where
  KeyChirho ('PackChirho (_ :: ProxyChirho keyChirho)) = keyChirho

data EqualChirho (leftChirho :: Type) (rightChirho :: Type) where
  ReflChirho :: EqualChirho valueChirho valueChirho

intProofChirho :: EqualChirho (KeyChirho ('PackChirho ('ProxyChirho :: ProxyChirho Int))) Int
intProofChirho = ReflChirho
boolProofChirho :: EqualChirho (KeyChirho ('PackChirho ('ProxyChirho :: ProxyChirho Bool))) Bool
boolProofChirho = ReflChirho

main :: IO ()
main = case intProofChirho of
  ReflChirho -> case boolProofChirho of
    ReflChirho -> putStrLn "distinct keys"

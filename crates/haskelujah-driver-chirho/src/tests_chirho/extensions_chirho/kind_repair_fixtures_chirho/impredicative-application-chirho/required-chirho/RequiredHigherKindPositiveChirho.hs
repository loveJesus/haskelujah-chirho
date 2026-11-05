-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RequiredTypeArguments, TypeAbstractions, KindSignatures #-}
module RequiredHigherKindPositiveChirho where
import Data.Kind (Type)
consumeChirho :: forall (fChirho :: Type -> Type) -> Int
consumeChirho _ = 0
valueChirho :: Int
valueChirho = consumeChirho Maybe

-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, StandaloneKindSignatures, TypeApplications, TypeAbstractions, DataKinds, RankNTypes #-}
module PhantomKindChirho where
import Data.Kind (Type)
type PhantomChirho :: forall indexChirho. Type
data PhantomChirho @indexChirho = PhantomChirho
badChirho :: PhantomChirho @Bool -> PhantomChirho @Type
badChirho valueChirho = valueChirho

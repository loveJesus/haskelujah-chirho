-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE ExplicitForAll, TypeApplications #-}
module LocalHigherKindIdChirho where
import Prelude hiding (id)
id :: forall f. f Int -> f Int
id xChirho = xChirho
valueChirho = id @Maybe

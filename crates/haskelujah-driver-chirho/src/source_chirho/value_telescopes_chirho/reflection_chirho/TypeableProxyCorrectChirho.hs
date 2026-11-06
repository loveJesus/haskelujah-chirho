-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module TypeableProxyCorrectChirho where
import qualified Data.Typeable as TypeableChirho
import Data.Proxy (Proxy(..))
valueChirho = TypeableChirho.typeRep (Proxy :: Proxy Int)

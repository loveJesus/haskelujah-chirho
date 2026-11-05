-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE ExplicitForAll, FlexibleInstances, PolyKinds, StandaloneKindSignatures, TypeFamilies #-}
module ClassAssocDefaultWrongChirho where
import Data.Kind (Type, Constraint)
import Data.Proxy (Proxy(..))
type CChirho :: forall kChirho. kChirho -> Constraint
class CChirho aChirho where
  type FChirho aChirho :: Type
  type FChirho aChirho = Proxy aChirho
instance CChirho Maybe
valueChirho :: FChirho Maybe
valueChirho = True

-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
-- GHC finds an imported module by its file name, so this file keeps the module's name.
module ReaderBaseChirho (ReaderChirho (..), runReaderChirho) where

newtype ReaderChirho r a = ReaderChirho (r -> a)

runReaderChirho :: ReaderChirho r a -> r -> a
runReaderChirho (ReaderChirho f) = f

instance Functor (ReaderChirho r) where
  fmap f (ReaderChirho g) = ReaderChirho (\r -> f (g r))

instance Applicative (ReaderChirho r) where
  pure x = ReaderChirho (\_ -> x)
  ReaderChirho f <*> ReaderChirho g = ReaderChirho (\r -> f r (g r))

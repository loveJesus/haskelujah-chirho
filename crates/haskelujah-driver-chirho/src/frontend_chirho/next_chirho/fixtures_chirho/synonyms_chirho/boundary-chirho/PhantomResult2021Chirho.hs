-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
module PhantomResult2021Chirho where
data PhantomChirho aChirho = PhantomChirho
data BoxChirho aChirho = BoxChirho aChirho
type AliasChirho = PhantomChirho
resultChirho :: AliasChirho BoxChirho
resultChirho = PhantomChirho

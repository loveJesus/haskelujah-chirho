-- TEST: compile_and_run
-- EXPECT_OUTPUT: 6
-- Typeclasses and instances
main = print (double (3 :: Int))

class Doubler a where
  double :: a -> a

instance Doubler Int where
  double x = x + x

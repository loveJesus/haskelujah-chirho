-- TEST: compile
-- Newtype declarations
module T014 where

newtype Name = MkName String

newtype Age = MkAge Int

newtype Wrapper a = Wrap a

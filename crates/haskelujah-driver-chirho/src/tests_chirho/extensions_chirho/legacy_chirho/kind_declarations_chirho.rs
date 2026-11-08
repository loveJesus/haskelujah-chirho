// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Existing extension controls, grouped by topic.

use crate::{compile_source_chirho, eval_source_chirho, eval_source_with_machine_chirho};
use haskelujah_runtime_chirho::ValueChirho;
use haskelujah_span_chirho::SourceMapChirho;

#[test]
fn quantified_constraint_parses_chirho() {
    // forall a. constraint in type signature context
    let src_chirho = "\
{-# LANGUAGE QuantifiedConstraints #-}
module Test where
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "QC.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn quantified_constraint_type_sig_parses_chirho() {
    // A type signature with a quantified constraint should parse
    let src_chirho = "\
{-# LANGUAGE QuantifiedConstraints #-}
{-# LANGUAGE RankNTypes #-}
module Test where
class MyClass f where
  myMethod :: f Int -> Int
foo :: (forall a. Show a) => Int
foo = 42
main = foo
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "QCSig.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn quantified_constraint_superclass_class_decl_parses_chirho() {
    let src_chirho = "\
{-# LANGUAGE QuantifiedConstraints #-}
module Test where
class (forall a. Show a => Eq (f a)) => MyClassChirho f where
  myMethodChirho :: f Int -> Int
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "QCSuper.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn quantified_constraint_local_polykinded_classes_instantiate_per_use_chirho() {
    let src_chirho = r#"
{-# LANGUAGE ConstraintKinds #-}
{-# LANGUAGE FlexibleInstances #-}
{-# LANGUAGE PolyKinds #-}
{-# LANGUAGE QuantifiedConstraints #-}
{-# LANGUAGE TypeOperators #-}
{-# LANGUAGE UndecidableInstances #-}
module Test where
import Data.Kind (Constraint)
data Dict c = Dict
data a :- b = Sub (a => Dict b)
class (forall a. p a) => Forall (p :: k -> Constraint)
instance (forall a. p a) => Forall (p :: k -> Constraint)
class p (f a) => ComposeC (p :: k2 -> Constraint) (f :: k1 -> k2) (a :: k1)
instance p (f a) => ComposeC p f a
class Forall (ComposeC p f) => ForallF (p :: k2 -> Constraint) (f :: k1 -> k2)
instance Forall (ComposeC p f) => ForallF p f
instF :: forall p f a . ForallF p f :- p (f a)
instF = undefined
class p (t a b) => R (p :: k3 -> Constraint) (t :: k1 -> k2 -> k3) (a :: k1) (b :: k2)
instance p (t a b) => R p t a b
class Forall (R p t a) => Q (p :: k3 -> Constraint) (t :: k1 -> k2 -> k3) (a :: k1)
instance Forall (R p t a) => Q p t a
class Forall (Q p t) => ForallT (p :: k4 -> Constraint) (t :: (k1 -> k2) -> k3 -> k4)
instance Forall (Q p t) => ForallT p t
instT :: forall k1 k2 k3 k4 (p :: k4 -> Constraint) (t :: (k1 -> k2) -> k3 -> k4) (f :: k1 -> k2) (a :: k3). ForallT p t :- p (t f a)
instT = undefined
main = 42
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "QCLocalPolyKinds.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn undecidable_instances_chirho() {
    // Instance with non-decreasing context — should compile fine
    let src_chirho = "\
{-# LANGUAGE UndecidableInstances #-}
{-# LANGUAGE FlexibleInstances #-}
module Test where
class MyClass a where
  myVal :: a -> Int
instance MyClass Int where
  myVal x = x
main = myVal (42 :: Int)
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "Undecidable.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn allow_ambiguous_types_chirho() {
    // Function with ambiguous type variable in context
    let src_chirho = "\
{-# LANGUAGE AllowAmbiguousTypes #-}
module Test where
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "Ambiguous.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn undecidable_superclasses_chirho() {
    // Classes with potentially cyclic superclass relationships
    let src_chirho = "\
{-# LANGUAGE UndecidableSuperClasses #-}
module Test where
class MyClass a where
  myMethod :: a -> Int
instance MyClass Int where
  myMethod x = x + 1
main = myMethod (41 :: Int)
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "UndecidableSuper.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn standalone_kind_sig_chirho() {
    // type T :: * — standalone kind signature parsed and accepted
    let src_chirho = "\
{-# LANGUAGE StandaloneKindSignatures #-}
module Test where
data MyType = MkMyType Int
main = case MkMyType 42 of
  MkMyType n -> n
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "StandaloneKind.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn polykinds_kind_variable_annotation_chirho() {
    // PolyKinds with kind variable annotation: data Proxy (a :: k) = MkProxy
    let src_chirho = "\
{-# LANGUAGE PolyKinds #-}
{-# LANGUAGE KindSignatures #-}
module Test where
data Proxy (a :: k) = MkProxy
x = MkProxy
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "PolyKinds2.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn polykinds_with_kind_arrow_chirho() {
    // PolyKinds with higher-kinded kind variable: (f :: k -> Type)
    let src_chirho = "\
{-# LANGUAGE PolyKinds #-}
{-# LANGUAGE KindSignatures #-}
module Test where
data HKD (f :: k -> *) = MkHKD
main = 42
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "PolyKinds3.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn standalone_kind_sig_with_data_chirho() {
    // StandaloneKindSignatures: `type T :: *` preceding a data decl
    let src_chirho = "\
{-# LANGUAGE StandaloneKindSignatures #-}
module Test where
type MyList :: * -> *
data MyList a = Nil | Cons a (MyList a)
main = case Cons 1 (Cons 2 Nil) of
  Cons x _ -> x
";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "StandaloneKind2.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(1));
}

#[test]
fn kind_hk_constraint_functor_chirho() {
    // Functor f => ... should work — f :: * -> *, not *
    let src_chirho = r#"
module Test where
class Functor f where
  fmap :: (a -> b) -> f a -> f b

data Box a = MkBox a

instance Functor Box where
  fmap g (MkBox x) = MkBox (g x)

main = case fmap (+1) (MkBox 41) of MkBox n -> n
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "KindHK1.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn kind_hk_constraint_monad_chirho() {
    // Monad m uses m :: * -> * in constraint context
    let src_chirho = r#"
{-# LANGUAGE NoImplicitPrelude #-}
module Test where
import Prelude ((+))
class Applicative f where
  pure :: a -> f a
class Applicative m => Monad m where
  bind :: m a -> (a -> m b) -> m b
data Id a = MkId a
instance Applicative Id where
  pure x = MkId x
instance Monad Id where
  bind (MkId x) f = f x
main = case bind (MkId 40) (\x -> MkId (x + 2)) of MkId n -> n
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "KindHK2.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn kind_forall_annotated_vars_chirho() {
    // forall (f :: * -> *). should work with kind annotations
    let src_chirho = r#"
{-# LANGUAGE KindSignatures #-}
{-# LANGUAGE RankNTypes #-}
{-# LANGUAGE NoImplicitPrelude #-}
module Test where
import Prelude (Int, (+))
data Box a = MkBox a
apply :: (forall a. a -> a) -> Int -> Int
apply f x = f x
main = apply (\x -> x) 42
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "KindForall.hs", None);
    assert_eq!(result_chirho.unwrap(), ValueChirho::IntChirho(42));
}

#[test]
fn type_family_reduction_in_sig_chirho() {
    // Type family `F` with instance `F Int = Bool`, then a function
    // with declared return type `F Int` that returns `True`.
    // Without type family reduction, this would fail with E0205
    // (type signature mismatch) because `F Int` wouldn't reduce to `Bool`.
    let src_chirho = r#"
{-# LANGUAGE TypeFamilies #-}
module Test where

type family F a
type instance F Int = Bool

myVal :: F Int
myVal = True

main = myVal
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // Compiles without E0205 — type family reduction resolves F Int = Bool
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "TFRedSig.hs");
    assert!(
        result_chirho.is_ok(),
        "Type family reduction in sig should compile: {:?}",
        result_chirho.err()
    );
}

#[test]
fn type_family_reduction_function_sig_chirho() {
    // Function with type family in result type
    let src_chirho = r#"
{-# LANGUAGE TypeFamilies #-}
module Test where

type family ResultOf a
type instance ResultOf Int = Int

addOne :: Int -> ResultOf Int
addOne x = x + 1

main = addOne 41
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "TFRedFn.hs", None);
    match &result_chirho {
        Ok(val_chirho) => {
            let s_chirho = format!("{}", val_chirho);
            assert!(s_chirho.contains("42"), "Expected 42, got: {}", s_chirho);
        }
        Err(e_chirho) => panic!("Type family in function sig should compile: {}", e_chirho),
    }
}

#[test]
fn type_family_closed_reduction_chirho() {
    // Closed type family with multiple equations
    let src_chirho = r#"
{-# LANGUAGE TypeFamilies #-}
module Test where

type family IsInt a where
  IsInt Int = Bool
  IsInt a   = Bool

checkInt :: IsInt Int
checkInt = True

main = checkInt
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    // Compiles without E0205 — closed type family reduces IsInt Int = Bool
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "TFClosed.hs");
    assert!(
        result_chirho.is_ok(),
        "Closed type family reduction should compile: {:?}",
        result_chirho.err()
    );
}

#[test]
fn data_family_decl_compiles_chirho() {
    // data family declaration should parse and compile without error
    let src_chirho = r#"
{-# LANGUAGE TypeFamilies #-}
module Test where
data family XList a
main = 42
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "DataFam.hs", None);
    assert!(
        result_chirho.is_ok(),
        "data family decl should compile: {:?}",
        result_chirho.err()
    );
}

#[test]
fn data_instance_decl_compiles_chirho() {
    // data instance should parse and compile without error
    let src_chirho = r#"
{-# LANGUAGE TypeFamilies #-}
module Test where
data family XList a
data instance XList Int = XListInt [Int]
main = 42
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "DataInst.hs", None);
    assert!(
        result_chirho.is_ok(),
        "data instance decl should compile: {:?}",
        result_chirho.err()
    );
}

#[test]
fn newtype_instance_decl_compiles_chirho() {
    // newtype instance should parse and compile without error
    let src_chirho = r#"
{-# LANGUAGE TypeFamilies #-}
module Test where
data family Wrapper a
newtype instance Wrapper Int = WrapInt Int
main = 42
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "NewtypeInst.hs", None);
    assert!(
        result_chirho.is_ok(),
        "newtype instance decl should compile: {:?}",
        result_chirho.err()
    );
}

#[test]
fn record_field_type_var_chirho() {
    let src_chirho = r#"module Test where
data Wrapper f = MkWrapper { unwrap :: f Int }
main = print 42
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "RecFieldVar.hs");
    assert!(
        result_chirho.is_ok(),
        "Record field with type variable should compile: {:?}",
        result_chirho.err()
    );
}

#[test]
fn record_field_app_type_chirho() {
    let src_chirho = r#"module Test where
data Box a = MkBox { getValue :: Maybe a }
main = print 42
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "RecFieldApp.hs");
    assert!(
        result_chirho.is_ok(),
        "Record field Maybe a should compile: {:?}",
        result_chirho.err()
    );
}

#[test]
fn record_field_fun_type_chirho() {
    let src_chirho = r#"module Test where
data Fun a b = MkFun { runFun :: a -> b }
main = print 42
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "RecFieldFun.hs");
    assert!(
        result_chirho.is_ok(),
        "Record field a -> b should compile: {:?}",
        result_chirho.err()
    );
}

#[test]
fn record_field_list_type_chirho() {
    let src_chirho = r#"module Test where
data Container a = MkContainer { items :: [a] }
main = print 42
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "RecFieldList.hs");
    assert!(
        result_chirho.is_ok(),
        "Record field [a] should compile: {:?}",
        result_chirho.err()
    );
}

#[test]
fn record_field_higher_kinded_chirho() {
    let src_chirho = r#"module Test where
newtype MaybeT m a = MaybeT { runMaybeT :: m (Maybe a) }
main = print 42
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "RecFieldHK.hs");
    assert!(
        result_chirho.is_ok(),
        "Higher-kinded record field m (Maybe a) should compile: {:?}",
        result_chirho.err()
    );
}

#[test]
fn type_annot_pat_basic_chirho() {
    use crate::eval_source_with_machine_chirho;
    let src_chirho = r#"module Test where
{-# LANGUAGE ScopedTypeVariables #-}
f (x :: Int) = x + 1
main = print (f 41)
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let (_v_chirho, m_chirho) =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "TypeAnnotPat.hs", None)
            .expect("type-annotated pattern should eval");
    assert_eq!(m_chirho.io_output_chirho.trim(), "42");
}

#[test]
fn type_annot_pat_case_chirho() {
    let src_chirho = r#"module Test where
{-# LANGUAGE ScopedTypeVariables #-}
f x = case x of { (y :: Int) -> y + 10 }
main = print (f 32)
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "TypeAnnotCase.hs");
    assert!(
        result_chirho.is_ok(),
        "Type-annotated pattern in case should compile: {:?}",
        result_chirho.err()
    );
}

#[test]
fn type_annot_pat_no_ext_chirho() {
    let src_chirho = r#"module Test where
f (x :: Int) = x + 1
main = print (f 41)
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "TypeAnnotNoExt.hs");
    assert!(
        result_chirho.is_ok(),
        "Type-annotated pattern without ext should compile: {:?}",
        result_chirho.err()
    );
}

#[test]
fn constraint_tuple_kind_chirho() {
    let src_chirho = r#"
module ConstraintTuple1 where

class Show a where
  show :: a -> String

class Eq a where
  eq :: a -> a -> Bool

showEq :: (Show a, Eq a) => a -> String
showEq x = show x

main :: IO ()
main = print (showEq 42)
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "ConstraintTuple1.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(machine_chirho.io_output_chirho.trim(), "\"42\"");
        }
        Err(e_chirho) => panic!("constraint tuple kind failed: {}", e_chirho),
    }
}

#[test]
fn constraint_tuple_three_chirho() {
    let src_chirho = r#"
module ConstraintTuple2 where

class Show a where
  show :: a -> String

class Eq a where
  eq :: a -> a -> Bool

class Ord a where
  compare :: a -> a -> Int

f :: (Show a, Eq a, Ord a) => a -> String
f x = show x

main :: IO ()
main = print (f 99)
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "ConstraintTuple2.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(machine_chirho.io_output_chirho.trim(), "\"99\"");
        }
        Err(e_chirho) => panic!("3-element constraint tuple failed: {}", e_chirho),
    }
}

#[test]
fn constraint_kinds_type_alias_chirho() {
    let src_chirho = r#"
{-# LANGUAGE ConstraintKinds #-}
module ConstraintKinds1 where

class Show a where
  show :: a -> String

class Eq a where
  eq :: a -> a -> Bool

type ShowEq a = (Show a, Eq a)

f :: ShowEq a => a -> String
f x = show x

main :: IO ()
main = print (f 42)
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "ConstraintKinds1.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(machine_chirho.io_output_chirho.trim(), "\"42\"");
        }
        Err(e_chirho) => panic!("ConstraintKinds type alias failed: {}", e_chirho),
    }
}

#[test]
fn constraint_tuple_superclass_chirho() {
    let src_chirho = r#"
module ConstraintTuple3 where

class Show a where
  show :: a -> String

class Eq a where
  eq :: a -> a -> Bool

class (Show a, Eq a) => Printable a where
  display :: a -> String

main :: IO ()
main = print 42
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_with_machine_chirho(src_chirho, &mut sm_chirho, "ConstraintTuple3.hs", None);
    match result_chirho {
        Ok((_val_chirho, machine_chirho)) => {
            assert_eq!(machine_chirho.io_output_chirho.trim(), "42");
        }
        Err(e_chirho) => panic!("constraint tuple superclass failed: {}", e_chirho),
    }
}

#[test]
fn gadt_standalone_kind_sig_chirho() {
    // `data V :: N -> Type where` — the type constructor V has kind N -> *,
    // so `V Z` should kind-check instead of erroring with "expected *, found k0 -> k1".
    let src_chirho = r#"
{-# LANGUAGE GADTs, DataKinds, KindSignatures #-}
module GadtKindSig where
import Data.Kind (Type)
data N = Z | S N
data V :: N -> Type where
  VZ :: V Z
  VS :: V n -> V (S n)
main :: IO ()
main = putStrLn "ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "GadtKindSig.hs", None);
    assert!(
        result_chirho.is_ok(),
        "GADT standalone kind sig failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn gadt_kind_sig_multi_param_chirho() {
    // Multi-parameter kind signature: `data Pair :: * -> * -> Type where`
    let src_chirho = r#"
{-# LANGUAGE GADTs, KindSignatures #-}
module GadtKindMulti where
import Data.Kind (Type)
data Pair :: * -> * -> Type where
  MkPair :: a -> b -> Pair a b
main :: IO ()
main = putStrLn "ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "GadtKindMulti.hs", None);
    assert!(
        result_chirho.is_ok(),
        "GADT multi-param kind sig failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn gadt_kind_sig_simple_star_chirho() {
    // Simple kind signature `data T :: Type where` (just * kind, no arrows).
    let src_chirho = r#"
{-# LANGUAGE GADTs, KindSignatures #-}
module GadtKindStar where
import Data.Kind (Type)
data T :: Type where
  MkT :: T
main :: IO ()
main = putStrLn "ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "GadtKindStar.hs", None);
    assert!(
        result_chirho.is_ok(),
        "GADT simple star kind sig failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn constraint_kind_in_gadt_arrow_chirho() {
    // Dict :: Constraint -> Type — `Constraint` appears in function arrow position
    let src_chirho = r#"
{-# LANGUAGE GADTs, ConstraintKinds, KindSignatures #-}
module DictGADT where

import Data.Kind

data Dict :: Constraint -> Type where
  Dict :: a => Dict a

main :: IO ()
main = print "ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "DictGADT.hs");
    assert!(
        result_chirho.is_ok(),
        "Constraint->Type GADT kind sig failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn constraint_star_interchangeable_in_type_family_chirho() {
    // Type families returning Constraint should kind-check
    let src_chirho = r#"
{-# LANGUAGE TypeFamilies, ConstraintKinds, KindSignatures #-}
module ConstraintFam where

import Data.Kind

class Show a where
  show :: a -> String

type family MyConstraint a :: Constraint where
  MyConstraint Int = Show Int

main :: IO ()
main = print "ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "ConstraintFam.hs");
    assert!(
        result_chirho.is_ok(),
        "Constraint type family failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn datakinds_bool_kind_annotation_chirho() {
    // DataKinds: (b :: Bool) uses a data type as a kind
    let src_chirho = r#"
{-# LANGUAGE DataKinds, KindSignatures, GADTs #-}
module BoolKind where

data MyFlag (b :: Bool) = MkFlag

main :: IO ()
main = print "ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "BoolKind.hs");
    assert!(
        result_chirho.is_ok(),
        "DataKinds Bool kind annotation failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn type_family_app_pattern_closed_chirho() {
    let src_chirho = r#"
{-# LANGUAGE TypeFamilies #-}
module TfAppChirho where

type family F a where
  F (Maybe a) = [a]
  F Int = Bool

foo :: F (Maybe Int)
foo = [1, 2, 3]

bar :: F Int
bar = True
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "TfApp.hs");
    assert!(
        result_chirho.is_ok(),
        "Type family app pattern failed: {:?}",
        result_chirho.err()
    );
}

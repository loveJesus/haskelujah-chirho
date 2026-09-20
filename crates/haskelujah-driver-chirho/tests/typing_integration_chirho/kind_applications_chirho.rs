// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Source kind-argument contracts, independently checked with GHC 9.14.1.
use super::{assert_compile_success_chirho, assert_execution_chirho};
use haskelujah_driver::typecheck_source_chirho;
use haskelujah_span_chirho::SourceMapChirho;

#[test]
fn newtype_standard_monad_contracts_chirho() {
    let source_chirho = r####"{-# LANGUAGE GeneralizedNewtypeDeriving, KindSignatures, PolyKinds #-}
module StandardMonadsChirho where
import Control.Monad.IO.Class
import Control.Monad.Fix
import Control.Monad.Trans.Class
import Control.Monad.Trans.Identity
newtype WrappedChirho (tagChirho :: kChirho) mChirho aChirho = WrappedChirho (IdentityT mChirho aChirho)
  deriving (Functor, Applicative, Monad, MonadIO, MonadFix, MonadTrans)
"####;
    assert_compile_success_chirho("standard_monads_chirho.hs", source_chirho);
}

#[test]
fn newtype_standard_monad_constraints_chirho() {
    let source_chirho = r####"{-# LANGUAGE FlexibleContexts #-}
module StandardConstraintsChirho where
import Control.Monad.IO.Class
import Control.Monad.Fix
import Control.Monad.Trans.Class
import Control.Monad.Trans.Identity
ioChirho :: MonadIO IO => Int
ioChirho = 1
fixChirho :: MonadFix IO => Int
fixChirho = 2
transChirho :: MonadTrans IdentityT => Int
transChirho = 3
"####;
    assert_compile_success_chirho("standard_constraints_chirho.hs", source_chirho);
}

#[test]
fn newtype_standard_monad_wrong_kind_chirho() {
    for (class_chirho, argument_chirho) in [
        ("MonadIO", "Int"),
        ("MonadFix", "Int"),
        ("MonadTrans", "IO"),
    ] {
        let source_chirho = format!(
            r####"{{-# LANGUAGE FlexibleContexts #-}}
module StandardWrongKindChirho where
import Control.Monad.IO.Class
import Control.Monad.Fix
import Control.Monad.Trans.Class
badChirho :: {class_chirho} {argument_chirho} => Int
badChirho = 1
"####
        );
        let message_chirho = rejection_chirho(&source_chirho);
        assert!(message_chirho.contains("kind mismatch"), "{message_chirho}");
    }
}

#[test]
fn newtype_standard_monad_local_shadow_chirho() {
    let source_chirho = r####"{-# LANGUAGE GeneralizedNewtypeDeriving, KindSignatures, FlexibleInstances #-}
module LocalMonadsChirho where
import Data.Kind (Type)
class MonadIO (aChirho :: Type)
class MonadFix (aChirho :: Type)
class MonadTrans (aChirho :: Type)
instance MonadIO Int
instance MonadFix Int
instance MonadTrans Int
newtype WrappedChirho = WrappedChirho Int deriving (MonadIO, MonadFix, MonadTrans)
"####;
    assert_compile_success_chirho("local_monads_chirho.hs", source_chirho);
}

#[test]
fn newtype_t12734_standard_monad_kinds_chirho() {
    let source_chirho = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../ghc-tests-chirho/typecheck-chirho/should_compile/T12734.hs"
    ));
    assert_compile_success_chirho("T12734.hs", source_chirho);
}

#[test]
fn newtype_stock_metadata_chirho() {
    let source_chirho = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DeriveDataTypeable, DeriveLift #-}
module StockMetadataChirho where
import Data.Data
import Language.Haskell.TH.Syntax (Lift)
newtype ConstantChirho aChirho bChirho = ConstantChirho aChirho deriving (Data, Typeable, Lift)
"####;
    assert_compile_success_chirho("stock_metadata_chirho.hs", source_chirho);
}

#[test]
fn newtype_stock_generic1_chirho() {
    let source_chirho = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DeriveGeneric #-}
module StockGeneric1Chirho where
import GHC.Generics
newtype ExceptChirho eChirho mChirho aChirho = ExceptChirho (mChirho (Either eChirho aChirho)) deriving (Generic, Generic1)
newtype TaggedChirho sChirho bChirho = TaggedChirho bChirho deriving (Generic, Generic1)
"####;
    assert_compile_success_chirho("stock_generic1_chirho.hs", source_chirho);
}

#[test]
fn newtype_ix_chirho() {
    let source_chirho = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE Haskell2010 #-}
module IxChirho where
import Data.Ix
newtype IndexChirho aChirho = IndexChirho aChirho deriving (Eq, Ord, Ix)
"####;
    assert_compile_success_chirho("ix_chirho.hs", source_chirho);
}

#[test]
fn newtype_ix_wrong_kind_chirho() {
    let source_chirho = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE FlexibleContexts #-}
module IxWrongKindChirho where
import Data.Ix
badChirho :: Ix Maybe => Int
badChirho = 1
"####;
    assert!(rejection_chirho(source_chirho).contains("kind mismatch"));
}

#[test]
fn newtype_ix_local_chirho() {
    let source_chirho = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GeneralizedNewtypeDeriving, KindSignatures, FlexibleInstances #-}
module LocalIxChirho where
import Data.Kind (Type)
class Ix (fChirho :: Type -> Type)
instance Ix []
newtype WrappedChirho aChirho = WrappedChirho [aChirho] deriving Ix
"####;
    assert_compile_success_chirho("ix_local_chirho.hs", source_chirho);
}

#[test]
fn newtype_imported_class_kind_checks_written_arguments_chirho() {
    let consumer_chirho = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GeneralizedNewtypeDeriving, FlexibleInstances, MultiParamTypeClasses #-}
module ConsumerChirho where
import qualified ProbeChirho as PChirho
newtype TChirho aChirho xChirho = TChirho (PChirho.RChirho aChirho xChirho)
  deriving (Functor, Applicative, Monad, PChirho.CChirho aChirho)
"####;
    for (source_chirho, accepted_chirho) in [
        (consumer_chirho.to_owned(), true),
        (
            consumer_chirho.replace("PChirho.CChirho aChirho)", "PChirho.CChirho Maybe)"),
            false,
        ),
    ] {
        let result_chirho = haskelujah_driver::compile_modules_chirho(
            &[
                ("ProbeChirho.hs", TYPED_READER_PRELUDE_CHIRHO),
                ("ConsumerChirho.hs", &source_chirho),
            ],
            &mut SourceMapChirho::new_chirho(),
        );
        if accepted_chirho {
            assert!(result_chirho.is_ok(), "{:?}", result_chirho.err());
        } else {
            let Err(error_chirho) = result_chirho else {
                panic!("the imported class fixes its first argument at Type");
            };
            let error_chirho = error_chirho.to_string();
            assert!(error_chirho.contains("kind mismatch"), "{error_chirho}");
        }
    }
}

// GND checks full applications after declaration kinds close. These controls
// establish frontend contracts, not runtime method coercion.
const READER_PRELUDE_CHIRHO: &str = r####"{-# LANGUAGE GeneralizedNewtypeDeriving, MultiParamTypeClasses, FlexibleInstances #-}
module ProbeChirho where

class Monad mChirho => CChirho rChirho mChirho

newtype RChirho rChirho aChirho = RChirho (rChirho -> aChirho)

instance Functor (RChirho rChirho) where
  fmap fChirho (RChirho gChirho) = RChirho (fChirho . gChirho)
instance Applicative (RChirho rChirho) where
  pure xChirho = RChirho (const xChirho)
  RChirho fChirho <*> RChirho gChirho = RChirho (\rChirho -> fChirho rChirho (gChirho rChirho))
instance Monad (RChirho rChirho) where
  RChirho gChirho >>= kChirho =
    RChirho (\rChirho -> case kChirho (gChirho rChirho) of RChirho hChirho -> hChirho rChirho)
instance CChirho rChirho (RChirho rChirho)

"####;
const TYPED_READER_PRELUDE_CHIRHO: &str = r####"{-# LANGUAGE GeneralizedNewtypeDeriving, MultiParamTypeClasses, FlexibleInstances, KindSignatures #-}
module ProbeChirho where
import Data.Kind (Type)

class Monad mChirho => CChirho (rChirho :: Type) mChirho

newtype RChirho rChirho aChirho = RChirho (rChirho -> aChirho)
instance Functor (RChirho rChirho) where
  fmap fChirho (RChirho gChirho) = RChirho (fChirho . gChirho)
instance Applicative (RChirho rChirho) where
  pure xChirho = RChirho (const xChirho)
  RChirho fChirho <*> RChirho gChirho = RChirho (\rChirho -> fChirho rChirho (gChirho rChirho))
instance Monad (RChirho rChirho) where
  RChirho gChirho >>= kChirho =
    RChirho (\rChirho -> case kChirho (gChirho rChirho) of RChirho hChirho -> hChirho rChirho)
instance CChirho rChirho (RChirho rChirho)

"####;

#[test]
fn newtype_arrow_chirho() {
    let source_chirho = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GeneralizedNewtypeDeriving, KindSignatures, FlexibleInstances #-}
module ArrowChirho where
import Data.Kind (Type)
class CChirho (fChirho :: Type -> Type)
instance CChirho ((->) Int)
newtype TChirho aChirho = TChirho (Int -> aChirho) deriving CChirho
"####;
    let result_chirho = typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "arrow_chirho.hs",
    );
    assert!(result_chirho.is_ok(), "{:?}", result_chirho.err());
}

#[test]
fn newtype_prefix_chirho() {
    let source_chirho = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GeneralizedNewtypeDeriving, KindSignatures, FlexibleInstances #-}
module PrefixChirho where
import Data.Kind (Type)
class CChirho (fChirho :: Type -> Type)
newtype TChirho aChirho = TChirho (aChirho -> aChirho) deriving CChirho
"####;
    let result_chirho = typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "prefix_chirho.hs",
    );
    let error_chirho = result_chirho
        .err()
        .expect("GHC rejects a free prefix parameter")
        .to_string();
    assert!(error_chirho.contains("cannot eta-reduce"), "{error_chirho}");
}

#[test]
fn newtype_two_kinds_chirho() {
    let source_chirho = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GeneralizedNewtypeDeriving, KindSignatures, FlexibleInstances, PolyKinds, DataKinds #-}
module TwoKindsChirho where
import Data.Kind (Type)
class CChirho (fChirho :: kChirho -> Type) where
  tagChirho :: fChirho aChirho -> Int
data PChirho (aChirho :: kChirho) = PChirho
instance CChirho PChirho where tagChirho _ = 7
newtype TChirho (aChirho :: kChirho) = TChirho (PChirho aChirho) deriving CChirho
firstChirho :: Int
firstChirho = tagChirho (TChirho PChirho :: TChirho Int)
secondChirho :: Int
secondChirho = tagChirho (TChirho PChirho :: TChirho 'True)
"####;
    let result_chirho = typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "two_kinds_chirho.hs",
    );
    assert!(result_chirho.is_ok(), "{:?}", result_chirho.err());
}

#[test]
fn newtype_list_chirho() {
    let source_chirho = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GeneralizedNewtypeDeriving, KindSignatures, FlexibleInstances #-}
module ListChirho where
import Data.Kind (Type)
class CChirho (fChirho :: Type -> Type)
instance CChirho []
newtype TChirho aChirho = TChirho [aChirho] deriving CChirho
"####;
    let result_chirho = typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "list_chirho.hs",
    );
    assert!(result_chirho.is_ok(), "{:?}", result_chirho.err());
}

#[test]
fn newtype_tuple_chirho() {
    let source_chirho = r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE GeneralizedNewtypeDeriving, KindSignatures, FlexibleInstances #-}
module TupleChirho where
import Data.Kind (Type)
class CChirho (fChirho :: Type -> Type)
instance CChirho ((,) Int)
newtype TChirho aChirho = TChirho (Int, aChirho) deriving CChirho
"####;
    let result_chirho = typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "tuple_chirho.hs",
    );
    assert!(result_chirho.is_ok(), "{:?}", result_chirho.err());
}

#[test]
fn newtype_t3955_full_source_chirho() {
    let source_chirho = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../ghc-tests-chirho/typecheck-chirho/should_compile/T3955.hs"
    ));
    let result_chirho = typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "T3955.hs",
    );
    assert!(result_chirho.is_ok(), "{:?}", result_chirho.err());
}

#[test]
fn newtype_g1_positive_chirho() {
    let source_chirho = [
        READER_PRELUDE_CHIRHO,
        r####"newtype T1Chirho aChirho xChirho = T1Chirho (RChirho aChirho xChirho)
  deriving (Functor, Applicative, Monad, CChirho aChirho)
"####,
    ]
    .concat();
    let result_chirho = typecheck_source_chirho(
        source_chirho.as_ref(),
        &mut SourceMapChirho::new_chirho(),
        "G1_positiveChirho.hs",
    );
    assert!(result_chirho.is_ok(), "{:?}", result_chirho.err());
}

#[test]
fn newtype_g2_second_newtype_chirho() {
    let source_chirho = [
        READER_PRELUDE_CHIRHO,
        r####"newtype T2Chirho bChirho yChirho = T2Chirho (RChirho bChirho yChirho)
  deriving (Functor, Applicative, Monad, CChirho bChirho)
"####,
    ]
    .concat();
    let result_chirho = typecheck_source_chirho(
        source_chirho.as_ref(),
        &mut SourceMapChirho::new_chirho(),
        "G2_second_newtypeChirho.hs",
    );
    assert!(result_chirho.is_ok(), "{:?}", result_chirho.err());
}

#[test]
fn newtype_g3_argument_omitted_chirho() {
    let source_chirho = [
        READER_PRELUDE_CHIRHO,
        r####"newtype T1Chirho aChirho xChirho = T1Chirho (RChirho aChirho xChirho)
  deriving (Functor, Applicative, Monad, CChirho)
"####,
    ]
    .concat();
    let result_chirho = typecheck_source_chirho(
        source_chirho.as_ref(),
        &mut SourceMapChirho::new_chirho(),
        "G3_argument_omittedChirho.hs",
    );
    let error_chirho = result_chirho
        .err()
        .expect("GHC rejects this declaration")
        .to_string();
    assert!(
        error_chirho.contains("not a unary constraint"),
        "{error_chirho}"
    );
}

#[test]
fn newtype_g4_inferred_kind_accepts_higher_kinded_argument_chirho() {
    let source_chirho = [
        READER_PRELUDE_CHIRHO,
        r####"newtype T1Chirho aChirho xChirho = T1Chirho (RChirho aChirho xChirho)
  deriving (Functor, Applicative, Monad, CChirho Maybe)
"####,
    ]
    .concat();
    let result_chirho = typecheck_source_chirho(
        source_chirho.as_ref(),
        &mut SourceMapChirho::new_chirho(),
        "G4_wrong_kind_argumentChirho.hs",
    );
    assert!(result_chirho.is_ok(), "{:?}", result_chirho.err());
}

#[test]
fn newtype_g5_un_eta_reducible_chirho() {
    let source_chirho = [
        READER_PRELUDE_CHIRHO,
        r####"newtype T3Chirho aChirho xChirho = T3Chirho (RChirho aChirho (xChirho, xChirho))
  deriving (CChirho aChirho)
"####,
    ]
    .concat();
    let result_chirho = typecheck_source_chirho(
        source_chirho.as_ref(),
        &mut SourceMapChirho::new_chirho(),
        "G5_un_eta_reducibleChirho.hs",
    );
    let error_chirho = result_chirho
        .err()
        .expect("GHC rejects this declaration")
        .to_string();
    assert!(error_chirho.contains("cannot eta-reduce"), "{error_chirho}");
}

#[test]
fn newtype_g6_un_eta_no_deriving_chirho() {
    let source_chirho = [
        READER_PRELUDE_CHIRHO,
        r####"newtype T3Chirho aChirho xChirho = T3Chirho (RChirho aChirho (xChirho, xChirho))
"####,
    ]
    .concat();
    let result_chirho = typecheck_source_chirho(
        source_chirho.as_ref(),
        &mut SourceMapChirho::new_chirho(),
        "G6_un_eta_no_derivingChirho.hs",
    );
    assert!(result_chirho.is_ok(), "{:?}", result_chirho.err());
}

#[test]
fn newtype_g7_wrong_kind_explicit_chirho() {
    let source_chirho = [
        TYPED_READER_PRELUDE_CHIRHO,
        r####"newtype T1Chirho aChirho xChirho = T1Chirho (RChirho aChirho xChirho)
  deriving (Functor, Applicative, Monad, CChirho Maybe)
"####,
    ]
    .concat();
    let result_chirho = typecheck_source_chirho(
        source_chirho.as_ref(),
        &mut SourceMapChirho::new_chirho(),
        "G7_wrong_kind_explicitChirho.hs",
    );
    let error_chirho = result_chirho
        .err()
        .expect("GHC rejects this declaration")
        .to_string();
    assert!(error_chirho.contains("kind mismatch"), "{error_chirho}");
}

#[test]
fn newtype_g8_kindsig_control_chirho() {
    let source_chirho = [
        TYPED_READER_PRELUDE_CHIRHO,
        r####"newtype T1Chirho aChirho xChirho = T1Chirho (RChirho aChirho xChirho)
  deriving (Functor, Applicative, Monad, CChirho aChirho)
"####,
    ]
    .concat();
    let result_chirho = typecheck_source_chirho(
        source_chirho.as_ref(),
        &mut SourceMapChirho::new_chirho(),
        "G8_kindsig_controlChirho.hs",
    );
    assert!(result_chirho.is_ok(), "{:?}", result_chirho.err());
}

const FAMILY_CHIRHO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../test-data-chirho/kind-oracles-chirho/visible-applications-chirho/FamilyKindApplicationsChirho.hs"
));
const ORDER_CHIRHO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../test-data-chirho/kind-oracles-chirho/visible-applications-chirho/KindBinderOrderChirho.hs"
));
const INFERRED_CHIRHO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../test-data-chirho/kind-oracles-chirho/visible-applications-chirho/InferredKindChirho.hs"
));
const AUTO_CHIRHO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../test-data-chirho/kind-oracles-chirho/visible-applications-chirho/AutoKindChirho.hs"
));
const PHANTOM_CHIRHO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../test-data-chirho/kind-oracles-chirho/visible-applications-chirho/PhantomKindChirho.hs"
));
const CONSTRUCTOR_CHIRHO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../test-data-chirho/kind-oracles-chirho/visible-applications-chirho/ConstructorKindChirho.hs"
));
const ANNOTATED_CHIRHO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../test-data-chirho/kind-oracles-chirho/visible-applications-chirho/AnnotatedKindChirho.hs"
));
const RECURSIVE_CHIRHO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../test-data-chirho/kind-oracles-chirho/visible-applications-chirho/RecursiveKindChirho.hs"
));
const LOCAL_CHIRHO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../test-data-chirho/kind-oracles-chirho/visible-applications-chirho/LocalKindChirho.hs"
));
const HIGHER_BINDER_CHIRHO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../test-data-chirho/kind-oracles-chirho/visible-applications-chirho/HigherKindBinderChirho.hs"
));
const REFINED_CHIRHO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../test-data-chirho/kind-oracles-chirho/visible-applications-chirho/RefinedKindChirho.hs"
));
const SYNONYM_CHIRHO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../test-data-chirho/kind-oracles-chirho/visible-applications-chirho/SynonymKindChirho.hs"
));
const CLASSIFIER_CHIRHO: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../test-data-chirho/kind-oracles-chirho/visible-applications-chirho/KindClassifierChirho.hs"
));
const DEFAULTED_SYNONYM_CHIRHO: &str = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE Haskell2010, EmptyDataDecls #-}
module DefaultedSynonymChirho where
data BoxChirho valueChirho
data InnerChirho valueChirho
type AliasChirho valueChirho = BoxChirho (InnerChirho valueChirho)
preserveChirho :: BoxChirho (InnerChirho Int) -> AliasChirho Int
preserveChirho valueChirho = valueChirho
"#;

fn rejection_chirho(source_chirho: &str) -> String {
    typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "InvalidKindApplicationChirho.hs",
    )
    .err()
    .unwrap_or_else(|| {
        panic!("this independently rejected kind-argument contract must fail:\n{source_chirho}")
    })
    .to_string()
}

#[test]
fn synonym_contracts_close_anonymous_kind_identities_chirho() {
    let source_chirho = r#"{-# LANGUAGE RankNTypes, PolyKinds #-}
module AnonymousSynonymChirho where
import Data.Kind (Type)
data TokenChirho (valueChirho :: kindChirho) = TokenChirho
data StoreChirho (polyChirho :: forall kindChirho. kindChirho -> Type) = StoreChirho (AliasChirho TokenChirho)
type AliasChirho = (StoreChirho :: (forall kindChirho. kindChirho -> Type) -> Type)
"#;
    assert_compile_success_chirho("AnonymousSynonymChirho.hs", source_chirho);
    let renamed_chirho = source_chirho.replace(
        "forall kindChirho. kindChirho -> Type",
        "forall innerChirho. innerChirho -> Type",
    );
    assert_compile_success_chirho("AnonymousSynonymChirho.hs", &renamed_chirho);
}

#[test]
fn qualified_kind_authority_survives_a_local_builtin_spelling_chirho() {
    let source_chirho = r#"{-# LANGUAGE DataKinds, KindSignatures #-}
module QualifiedKindChirho where
import qualified Data.Kind as KChirho
data Type = LocalTypeChirho
data BoxChirho (valueChirho :: KChirho.Type) = BoxChirho
type GoodChirho = BoxChirho Int
"#;
    assert_compile_success_chirho("QualifiedKindChirho.hs", source_chirho);
    let wrong_chirho = source_chirho.replace("BoxChirho Int", "BoxChirho 'LocalTypeChirho");
    assert!(rejection_chirho(&wrong_chirho).contains("kind mismatch"));
}

#[test]
fn nested_synonym_foralls_are_fresh_and_still_rigid_chirho() {
    let source_chirho = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes #-}
module Main where
type ResultChirho valueChirho = forall argumentChirho. argumentChirho -> valueChirho
answerChirho :: ResultChirho (ResultChirho Int)
answerChirho firstChirho secondChirho = 42
main = print (answerChirho 'c' True)
"#;
    assert_execution_chirho(source_chirho, "42\n");
    let wrong_chirho = source_chirho.replace(
        "secondChirho = 42",
        "secondChirho = if secondChirho then 42 else 0",
    );
    assert!(rejection_chirho(&wrong_chirho).contains("type mismatch"));
    let lambda_chirho = source_chirho.replace(
        "answerChirho firstChirho secondChirho = 42",
        "answerChirho = \\firstChirho secondChirho -> 42",
    );
    assert_execution_chirho(&lambda_chirho, "42\n");
    let wrong_lambda_chirho = lambda_chirho.replace(
        "secondChirho -> 42",
        "secondChirho -> if secondChirho then 42 else 0",
    );
    assert!(rejection_chirho(&wrong_lambda_chirho).contains("type mismatch"));
}

#[test]
fn inferred_hidden_arguments_solve_their_dependent_classifiers_chirho() {
    let source_chirho = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, PolyKinds #-}
module Main where
data ProxyChirho (valueChirho :: kindChirho) = ProxyChirho
data TagChirho (indexChirho :: ProxyChirho valueChirho) = TagChirho Int
type PairChirho (leftChirho :: ProxyChirho Either) (rightChirho :: ProxyChirho Maybe) = (TagChirho leftChirho, TagChirho rightChirho)
readChirho :: PairChirho 'ProxyChirho 'ProxyChirho -> Int
readChirho (TagChirho leftChirho, TagChirho rightChirho) = leftChirho + rightChirho
main = print (readChirho (TagChirho 40, TagChirho 2))
"#;
    assert_execution_chirho(source_chirho, "42\n");
}

#[test]
fn promoted_reflexivity_retains_the_kind_of_its_value_chirho() {
    let source_chirho = r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, PolyKinds, TypeOperators #-}
module PromotedReflexivityChirho where
import Data.Type.Equality
data ProofChirho (proofChirho :: True :~: True)
keepChirho :: ProofChirho Refl -> ProofChirho Refl
keepChirho valueChirho = valueChirho
"#;
    assert_compile_success_chirho("PromotedReflexivityChirho.hs", source_chirho);
    let wrong_chirho = source_chirho.replace("True :~: True", "True :~: False");
    assert!(rejection_chirho(&wrong_chirho).contains("kind mismatch"));
}

#[test]
fn visible_kind_argument_selects_the_family_classifier_chirho() {
    assert_compile_success_chirho("FamilyKindApplicationsChirho.hs", FAMILY_CHIRHO);
    let wrong_chirho = FAMILY_CHIRHO.replace("BoxChirho @Type Maybe", "BoxChirho @Type Int");
    assert!(rejection_chirho(&wrong_chirho).contains("kind mismatch"));
}

#[test]
fn visible_kind_arguments_follow_source_binder_order_chirho() {
    assert_compile_success_chirho("KindBinderOrderChirho.hs", ORDER_CHIRHO);
    let wrong_chirho = ORDER_CHIRHO.replace("@Type @Bool", "@Bool @Type");
    assert!(rejection_chirho(&wrong_chirho).contains("kind mismatch"));
}

#[test]
fn inferred_kind_binders_are_skipped_not_visibly_consumed_chirho() {
    assert_compile_success_chirho("InferredKindChirho.hs", INFERRED_CHIRHO);
    assert!(rejection_chirho(AUTO_CHIRHO).contains("kind argument"));
}

#[test]
fn visible_kind_arguments_are_name_checked_chirho() {
    let wrong_chirho = FAMILY_CHIRHO.replace("@Type Maybe", "@MissingKindChirho Maybe");
    assert!(rejection_chirho(&wrong_chirho).contains("MissingKindChirho"));
}

#[test]
fn phantom_kind_arguments_remain_distinct_during_type_equality_chirho() {
    let positive_chirho = PHANTOM_CHIRHO.replace("PhantomChirho @Type", "PhantomChirho @Bool");
    assert_compile_success_chirho("SamePhantomKindChirho.hs", &positive_chirho);
    assert!(rejection_chirho(PHANTOM_CHIRHO).contains("type mismatch"));
}

#[test]
fn explicit_kind_signatures_interoperate_with_implicit_constructor_arguments_chirho() {
    assert_execution_chirho(CONSTRUCTOR_CHIRHO, "42\n");
}

#[test]
fn wildcard_kind_arguments_are_inferred_from_the_ordinary_argument_chirho() {
    assert_execution_chirho(&CONSTRUCTOR_CHIRHO.replace("@Bool", "@_"), "42\n");
}

#[test]
fn recursive_nominal_fields_keep_the_group_kind_arguments_chirho() {
    assert_execution_chirho(RECURSIVE_CHIRHO, "42\n7\n");
}

#[test]
fn forall_annotated_kind_binders_instantiate_at_each_use_chirho() {
    assert_compile_success_chirho("HigherKindBinderChirho.hs", HIGHER_BINDER_CHIRHO);
    let monomorphic_chirho =
        HIGHER_BINDER_CHIRHO.replace("forall kindChirho. kindChirho -> Type", "Type -> Type");
    assert!(rejection_chirho(&monomorphic_chirho).contains("kind mismatch"));
}

#[test]
fn constructor_refinement_reaches_the_indexed_nominal_head_chirho() {
    assert_execution_chirho(REFINED_CHIRHO, "42\n");
    let no_equality_chirho = REFINED_CHIRHO.replace(
        "ReflChirho :: EqualChirho valueChirho valueChirho",
        "ReflChirho :: EqualChirho leftChirho rightChirho",
    );
    assert!(rejection_chirho(&no_equality_chirho).contains("type mismatch"));
}

#[test]
fn synonym_bodies_keep_solved_kind_arguments_and_parameter_sharing_chirho() {
    assert_execution_chirho(SYNONYM_CHIRHO, "42\n7\n11\n");
    let wrong_chirho = SYNONYM_CHIRHO.replace(
        "PhantomAliasChirho Bool -> PhantomChirho @Bool",
        "PhantomAliasChirho Bool -> PhantomChirho @Type",
    );
    assert!(rejection_chirho(&wrong_chirho).contains("type mismatch"));
}

#[test]
fn a_kind_annotation_requires_a_type_not_a_promoted_value_chirho() {
    assert_compile_success_chirho("KindClassifierChirho.hs", CLASSIFIER_CHIRHO);
    let wrong_chirho = CLASSIFIER_CHIRHO.replace("kindChirho :: Type", "kindChirho :: Bool");
    assert!(rejection_chirho(&wrong_chirho).contains("kind annotation"));
}

#[test]
fn monokinded_alias_bodies_share_the_defaulted_head_contract_chirho() {
    assert_compile_success_chirho("DefaultedSynonymChirho.hs", DEFAULTED_SYNONYM_CHIRHO);
    let wrong_chirho = DEFAULTED_SYNONYM_CHIRHO.replace("InnerChirho Int", "InnerChirho Maybe");
    assert!(rejection_chirho(&wrong_chirho).contains("kind mismatch"));
}

#[test]
fn expression_signatures_instantiate_nominal_kind_slots_without_erasing_them_chirho() {
    assert_execution_chirho(LOCAL_CHIRHO, "42\n");
    let wrong_chirho = LOCAL_CHIRHO.replace(
        "TokenChirho :: TokenChirho @Bool",
        "TokenChirho :: TokenChirho @Type",
    );
    assert!(rejection_chirho(&wrong_chirho).contains("type mismatch"));
}

#[test]
fn visible_kind_arguments_inside_binder_annotations_are_checked_chirho() {
    assert_compile_success_chirho("AnnotatedKindChirho.hs", ANNOTATED_CHIRHO);
    assert!(
        rejection_chirho(&ANNOTATED_CHIRHO.replace("@Type", "@MissingKindChirho"))
            .contains("MissingKindChirho")
    );
    assert!(
        rejection_chirho(&ANNOTATED_CHIRHO.replace("@Type", "@Bool")).contains("kind mismatch")
    );
}

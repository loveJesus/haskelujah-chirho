// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Existing extension controls, grouped by topic.

use crate::{compile_source_chirho, eval_source_chirho};
use haskelujah_span_chirho::SourceMapChirho;

#[test]
fn import_control_monad_trans_identity_chirho() {
    let src_chirho = "module Test where\nimport Control.Monad.Trans.Identity (IdentityT)\nf = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "TransIdentity.hs");
    assert!(
        result_chirho.is_ok(),
        "Control.Monad.Trans.Identity import: {:?}",
        result_chirho.err()
    );
}

#[test]
fn import_control_monad_trans_state_chirho() {
    let src_chirho = "module Test where\nimport Control.Monad.Trans.State (StateT)\nf = 42\n";
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "TransState.hs");
    assert!(
        result_chirho.is_ok(),
        "Control.Monad.Trans.State import: {:?}",
        result_chirho.err()
    );
}

#[test]
fn import_type_reflection_chirho() {
    let src_chirho = r#"
{-# LANGUAGE NoImplicitPrelude #-}
module Test where
import Type.Reflection (TypeRep, Typeable)
main = 42
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "TypeRefl.hs");
    assert!(
        result_chirho.is_ok(),
        "Type.Reflection import should work: {:?}",
        result_chirho.err()
    );
}

#[test]
fn import_type_reflection_rnf_module_chirho() {
    let src_chirho = r#"
module Test where
import qualified Type.Reflection as Reflection

f :: Reflection.Module -> ()
f = Reflection.rnfModule
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "TypeReflRnfModule.hs");
    assert!(
        result_chirho.is_ok(),
        "Type.Reflection should export rnfModule: {:?}",
        result_chirho.err()
    );
}

#[test]
fn import_unsafe_coerce_chirho() {
    let src_chirho = r#"
{-# LANGUAGE NoImplicitPrelude #-}
module Test where
import Unsafe.Coerce (unsafeCoerce)
main = 42
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "UnsCoerce.hs");
    assert!(
        result_chirho.is_ok(),
        "Unsafe.Coerce import should work: {:?}",
        result_chirho.err()
    );
}

#[test]
fn import_ghc_exception_chirho() {
    let src_chirho = r#"
{-# LANGUAGE NoImplicitPrelude #-}
module Test where
import GHC.Exception (SomeException, throw)
main = 42
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "GHCExc.hs");
    assert!(
        result_chirho.is_ok(),
        "GHC.Exception import should work: {:?}",
        result_chirho.err()
    );
}

#[test]
fn import_data_type_equality_chirho() {
    let src_chirho = r#"module Test where
import Data.Type.Equality (testEquality, castWith)
main = print 42
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "DataTypeEq.hs");
    assert!(
        result_chirho.is_ok(),
        "Data.Type.Equality import should compile: {:?}",
        result_chirho.err()
    );
}

#[test]
fn import_control_applicative_chirho() {
    let src_chirho = r#"module Test where
import Control.Applicative
f :: ZipList Int -> Int
f _ = 42
main = print 42
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "CtrlApp.hs");
    assert!(
        result_chirho.is_ok(),
        "Control.Applicative import should compile: {:?}",
        result_chirho.err()
    );
}

#[test]
fn import_data_functor_identity_chirho() {
    let src_chirho = r#"module Test where
import Data.Functor.Identity
f :: Identity Int -> Int
f (Identity x) = x
main = print (f (Identity 42))
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "FunId.hs");
    assert!(
        result_chirho.is_ok(),
        "Data.Functor.Identity import should compile: {:?}",
        result_chirho.err()
    );
}

#[test]
fn import_data_functor_identity_all_brings_runidentity_chirho() {
    let src_chirho = r#"module Test where
import Data.Functor.Identity (Identity(..))
f :: Identity Int -> Int
f = runIdentity
main = print (f (Identity 42))
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "FunIdAll.hs");
    assert!(
        result_chirho.is_ok(),
        "Identity(..) should bring runIdentity into scope: {:?}",
        result_chirho.err()
    );
}

#[test]
fn import_ghc_base_chirho() {
    let src_chirho = r#"module Test where
import GHC.Base (id, const, flip)
f = id 42
main = print f
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "GHCBase.hs");
    assert!(
        result_chirho.is_ok(),
        "GHC.Base import should compile: {:?}",
        result_chirho.err()
    );
}

#[test]
fn import_data_foldable_chirho() {
    let src_chirho = r#"module Test where
import Data.Foldable (toList, fold)
main = print 42
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "Foldable.hs");
    assert!(
        result_chirho.is_ok(),
        "Data.Foldable import should compile: {:?}",
        result_chirho.err()
    );
}

#[test]
fn import_data_ioref_chirho() {
    let src_chirho = r#"module Test where
import Data.IORef
main = print 42
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "IORef.hs");
    assert!(
        result_chirho.is_ok(),
        "Data.IORef import should compile: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_data_proxy_chirho() {
    let src_chirho = r#"
module IfaceProxy where
import Data.Proxy (Proxy(..))

proxyInt :: Proxy Int
proxyInt = Proxy

main :: IO ()
main = print "proxy ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceProxy.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Data.Proxy import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_data_typeable_chirho() {
    let src_chirho = r#"
module IfaceTypeable where
import Data.Typeable (Typeable, typeOf)

main :: IO ()
main = print "typeable ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceTypeable.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Data.Typeable import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_data_typeable_rnf_exports_chirho() {
    let src_chirho = r#"
module IfaceTypeableRnf where
import Data.Typeable (TyCon, TypeRep, rnfTyCon, rnfTypeRep)

f :: TyCon -> ()
f = rnfTyCon

g :: TypeRep a -> ()
g = rnfTypeRep
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "IfaceTypeableRnf.hs");
    assert!(
        result_chirho.is_ok(),
        "Data.Typeable should export rnfTyCon/rnfTypeRep: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_data_typeable_type_rep_proxy_arg_chirho() {
    let src_chirho = r#"
{-# LANGUAGE ScopedTypeVariables, TypeApplications #-}
module IfaceTypeableTypeRepProxy where
import Data.Proxy
import Data.Typeable (Typeable, typeRep)

showTypeRep :: forall t. Typeable t => String
showTypeRep = show $ typeRep (Proxy @t)
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        compile_source_chirho(src_chirho, &mut sm_chirho, "IfaceTypeableTypeRepProxy.hs");
    assert!(
        result_chirho.is_ok(),
        "Data.Typeable.typeRep should accept a proxy argument: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_ghc_typelits_charval_chirho() {
    let src_chirho = r#"
{-# LANGUAGE DataKinds, ScopedTypeVariables, TypeApplications #-}
module IfaceTypeLitsCharVal where
import Data.Proxy
import GHC.TypeLits

charName :: forall c. KnownChar c => Char
charName = charVal (Proxy @c)
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        compile_source_chirho(src_chirho, &mut sm_chirho, "IfaceTypeLitsCharVal.hs");
    assert!(
        result_chirho.is_ok(),
        "GHC.TypeLits should export charVal/KnownChar: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_test_hspec_minimal_spec_chirho() {
    let src_chirho = r#"
module IfaceHspec where
import Test.Hspec

spec :: Spec
spec = describe "stub" $ it "checks equality" $ (1 :: Int) `shouldBe` 1

main :: IO ()
main = hspec spec
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "IfaceHspec.hs");
    assert!(
        result_chirho.is_ok(),
        "Test.Hspec minimal spec should typecheck: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_data_coerce_chirho() {
    let src_chirho = r#"
module IfaceCoerce where
import Data.Coerce (Coercible, coerce)

main :: IO ()
main = print "coerce ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceCoerce.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Data.Coerce import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_ghc_type_lits_chirho() {
    let src_chirho = r#"
module IfaceTypeLits where
import GHC.TypeLits (Nat, Symbol, KnownNat, KnownSymbol, natVal, symbolVal)

main :: IO ()
main = print "typelits ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceTypeLits.hs", None);
    assert!(
        result_chirho.is_ok(),
        "GHC.TypeLits import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_ghc_type_nats_chirho() {
    let src_chirho = r#"
module IfaceTypeNats where
import GHC.TypeNats (Nat, KnownNat, natVal)

main :: IO ()
main = print "typenats ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceTypeNats.hs", None);
    assert!(
        result_chirho.is_ok(),
        "GHC.TypeNats import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_foreign_storable_chirho() {
    let src_chirho = r#"
module IfaceStorable where
import Foreign.Storable (Storable, sizeOf, alignment)

main :: IO ()
main = print "storable ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceStorable.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Foreign.Storable import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_foreign_ptr_chirho() {
    let src_chirho = r#"
module IfacePtr where
import Foreign.Ptr (Ptr, FunPtr, nullPtr, castPtr)

main :: IO ()
main = print "ptr ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfacePtr.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Foreign.Ptr import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_control_concurrent_mvar_chirho() {
    let src_chirho = r#"
module IfaceMVar where
import Control.Concurrent.MVar (MVar, newMVar, takeMVar, putMVar)

main :: IO ()
main = print "mvar ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceMVar.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Control.Concurrent.MVar import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_control_monad_trans_class_chirho() {
    let src_chirho = r#"
module IfaceMonadTrans where
import Control.Monad.Trans.Class (MonadTrans, lift)

main :: IO ()
main = print "monadtrans ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceMonadTrans.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Control.Monad.Trans.Class import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_text_prettyprint_chirho() {
    let src_chirho = r#"
module IfacePrettyPrint where
import Text.PrettyPrint (Doc, text, render)

main :: IO ()
main = print "prettyprint ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfacePrettyPrint.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Text.PrettyPrint import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_data_data_chirho() {
    let src_chirho = r#"
module IfaceDataData where
import Data.Data (Data, Typeable, toConstr)

main :: IO ()
main = print "data ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceDataData.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Data.Data import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_data_word_chirho() {
    let src_chirho = r#"
module IfaceWord where
import Data.Word (Word, Word8, Word16, Word32, Word64)

main :: IO ()
main = print "word ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceWord.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Data.Word import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_data_int_chirho() {
    let src_chirho = r#"
module IfaceDataInt where
import Data.Int (Int, Int8, Int16, Int32, Int64)

main :: IO ()
main = print "int ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceDataInt.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Data.Int import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_data_map_chirho() {
    let src_chirho = r#"
module IfaceDataMap where
import Data.Map (Map, empty, singleton, insert, lookup)

main :: IO ()
main = print "map ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceDataMap.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Data.Map import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_data_map_strict_chirho() {
    let src_chirho = r#"
module IfaceDataMapStrict where
import Data.Map.Strict (Map, empty, singleton, insert)

main :: IO ()
main = print "map strict ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceDataMapStrict.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Data.Map.Strict import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_data_set_chirho() {
    let src_chirho = r#"
module IfaceDataSet where
import Data.Set (Set, empty, singleton, insert, member)

main :: IO ()
main = print "set ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceDataSet.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Data.Set import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_ghc_generics_chirho() {
    let src_chirho = r#"
module IfaceGenerics where
import GHC.Generics (Generic, Rep, from, to)

main :: IO ()
main = print "generics ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceGenerics.hs", None);
    assert!(
        result_chirho.is_ok(),
        "GHC.Generics import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_debug_trace_chirho() {
    let src_chirho = r#"
module IfaceTrace where
import Debug.Trace (trace, traceShow)

main :: IO ()
main = print "trace ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceTrace.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Debug.Trace import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_data_bits_chirho() {
    let src_chirho = r#"
module IfaceBits where
import Data.Bits (Bits, (.&.), (.|.), xor, complement, shiftL, shiftR)

main :: IO ()
main = print "bits ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceBits.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Data.Bits import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_data_ratio_chirho() {
    let src_chirho = r#"
module IfaceRatio where
import Data.Ratio (Ratio, Rational, numerator, denominator)

main :: IO ()
main = print "ratio ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceRatio.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Data.Ratio import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_data_complex_chirho() {
    let src_chirho = r#"
module IfaceComplex where
import Data.Complex (Complex, realPart, imagPart, magnitude)

main :: IO ()
main = print "complex ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceComplex.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Data.Complex import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_system_exit_chirho() {
    let src_chirho = r#"
module IfaceExit where
import System.Exit (ExitCode(..), exitSuccess, exitFailure)

main :: IO ()
main = print "exit ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceExit.hs", None);
    assert!(
        result_chirho.is_ok(),
        "System.Exit import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_data_dynamic_chirho() {
    let src_chirho = r#"
module IfaceDynamic where
import Data.Dynamic (Dynamic, toDyn, fromDyn)

main :: IO ()
main = print "dynamic ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceDynamic.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Data.Dynamic import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_control_deepseq_chirho() {
    let src_chirho = r#"
module IfaceDeepSeq where
import Control.DeepSeq (NFData, deepseq, rnf, force)

main :: IO ()
main = print "deepseq ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceDeepSeq.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Control.DeepSeq import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_data_intmap_chirho() {
    let src_chirho = r#"
module IfaceIntMap where
import Data.IntMap (IntMap, empty, singleton, insert, lookup)

main :: IO ()
main = print "intmap ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceIntMap.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Data.IntMap import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_data_sequence_chirho() {
    let src_chirho = r#"
module IfaceSeq where
import Data.Sequence (Seq, empty, singleton)

main :: IO ()
main = print "sequence ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceSeq.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Data.Sequence import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_ghc_num_chirho() {
    let src_chirho = r#"
module IfaceGhcNum where
import GHC.Num (Num, subtract, fromInteger)

main :: IO ()
main = print "ghcnum ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceGhcNum.hs", None);
    assert!(
        result_chirho.is_ok(),
        "GHC.Num import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_ghc_real_chirho() {
    let src_chirho = r#"
module IfaceGhcReal where
import GHC.Real (Integral, toInteger, fromIntegral, quot, rem, div, mod)

main :: IO ()
main = print "ghcreal ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceGhcReal.hs", None);
    assert!(
        result_chirho.is_ok(),
        "GHC.Real import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_ghc_float_chirho() {
    let src_chirho = r#"
module IfaceGhcFloat where
import GHC.Float (Float, Double, floatDigits, isNaN, isInfinite)

main :: IO ()
main = print "ghcfloat ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceGhcFloat.hs", None);
    assert!(
        result_chirho.is_ok(),
        "GHC.Float import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_ghc_enum_chirho() {
    let src_chirho = r#"
module IfaceGhcEnum where
import GHC.Enum (Enum, Bounded, succ, pred, toEnum, fromEnum, minBound, maxBound)

main :: IO ()
main = print "ghcenum ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceGhcEnum.hs", None);
    assert!(
        result_chirho.is_ok(),
        "GHC.Enum import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_data_char_expanded_chirho() {
    let src_chirho = r#"
module IfaceCharExpanded where
import Data.Char (isLetter, isPrint, isAscii, toTitle)

main :: IO ()
main = print "char expanded ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceCharExpanded.hs", None);
    assert!(
        result_chirho.is_ok(),
        "Data.Char expanded import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_ghc_prim_expanded_chirho() {
    let src_chirho = r#"
module IfacePrimExpanded where
import GHC.Prim (dataToTag#, tagToEnum#, reallyUnsafePtrEquality#)

main :: IO ()
main = print "prim expanded ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho =
        eval_source_chirho(src_chirho, &mut sm_chirho, "IfacePrimExpanded.hs", None);
    assert!(
        result_chirho.is_ok(),
        "GHC.Prim expanded import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn iface_ghc_records_chirho() {
    let src_chirho = r#"
module IfaceRecords where
import GHC.Records (HasField, getField)

main :: IO ()
main = print "records ok"
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = eval_source_chirho(src_chirho, &mut sm_chirho, "IfaceRecords.hs", None);
    assert!(
        result_chirho.is_ok(),
        "GHC.Records import failed: {:?}",
        result_chirho.err()
    );
}

#[test]
fn imported_module_value_resolves_chirho() {
    let src_chirho = r#"
module ImportValChirho where
import Data.Map (singleton, empty)
import Foreign.Storable (sizeOf)
import Data.Foldable (foldl')

x :: Int
x = 0
"#;
    let mut sm_chirho = SourceMapChirho::new_chirho();
    let result_chirho = compile_source_chirho(src_chirho, &mut sm_chirho, "ImportVal.hs");
    assert!(
        result_chirho.is_ok(),
        "Imported module values should resolve: {:?}",
        result_chirho.err()
    );
}

#[test]
fn prelude_export_count_chirho() {
    let ifaces_chirho = haskelujah_naming_chirho::builtin_module_ifaces_chirho();
    let prelude_chirho = ifaces_chirho
        .iter()
        .find(|m_chirho| m_chirho.name_chirho == "Prelude")
        .unwrap();
    // Prelude should have substantial exports after expansion
    assert!(
        prelude_chirho.exports_chirho.values_chirho.len() > 200,
        "Prelude should have 200+ value exports"
    );
    assert!(
        prelude_chirho.exports_chirho.types_chirho.len() > 30,
        "Prelude should have 30+ type exports"
    );
}

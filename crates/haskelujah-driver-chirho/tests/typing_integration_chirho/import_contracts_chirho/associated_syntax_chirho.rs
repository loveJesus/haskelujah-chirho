// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
//! Unchanged GHC9.14.1 associated-result-binder sources with consumers.
use haskelujah_driver::typecheck_source_chirho;
use haskelujah_span_chirho::SourceMapChirho;

#[test]
fn associated_injectivity_uses_declared_result_and_parameter_binders_chirho() {
    for (annotation_chirho, reason_chirho) in [
        (
            "rChirho -> missingChirho",
            "injectivity names an unbound family parameter",
        ),
        (
            "otherChirho -> aChirho",
            "injectivity must name the declared result binder",
        ),
    ] {
        let source_chirho = format!(
            "{{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures #-}}\nmodule BadDependencyChirho where\nimport Data.Kind (Type)\nclass CChirho (aChirho :: Type) where\n  type FChirho aChirho = rChirho | {annotation_chirho}\n"
        );
        let errors_chirho = typecheck_source_chirho(
            &source_chirho,
            &mut SourceMapChirho::new_chirho(),
            "BadDependencyChirho.hs",
        )
        .err()
        .expect("an unbound annotation cannot become a checked contract");
        assert!(
            errors_chirho.to_string().contains(reason_chirho),
            "{errors_chirho}"
        );
    }
}

#[test]
fn s1_kinded_result_binder_typefamilies_chirho() {
    let source_chirho = r####"{-# LANGUAGE TypeFamilies, KindSignatures, MultiParamTypeClasses #-}
module ProbeChirho where
import Data.Kind (Type)
data X
data Y
class FrozenGen f m where
  type MutableGen f m = (g :: Type)
instance FrozenGen X Y where
  type MutableGen X Y = Int
use :: MutableGen X Y -> Int
use n = n
"####;
    let result_chirho = typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "ProbeChirho.hs",
    );
    let error_chirho = result_chirho
        .err()
        .expect("invalid declaration must be diagnosed")
        .to_string();
    assert!(
        error_chirho.contains("associated family result binder requires an injectivity annotation"),
        "{error_chirho}"
    );
}

#[test]
fn s1b_kinded_result_binder_typefamilydependencies_chirho() {
    let source_chirho = r####"{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures, MultiParamTypeClasses #-}
module ProbeChirho where
import Data.Kind (Type)
data X
data Y
class FrozenGen f m where
  type MutableGen f m = (g :: Type)
instance FrozenGen X Y where
  type MutableGen X Y = Int
use :: MutableGen X Y -> Int
use n = n
"####;
    let result_chirho = typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "ProbeChirho.hs",
    );
    let error_chirho = result_chirho
        .err()
        .expect("invalid declaration must be diagnosed")
        .to_string();
    assert!(
        error_chirho.contains("associated family result binder requires an injectivity annotation"),
        "{error_chirho}"
    );
}

#[test]
fn s2_bare_result_binder_chirho() {
    let source_chirho = r####"{-# LANGUAGE TypeFamilies, KindSignatures, MultiParamTypeClasses #-}
module ProbeChirho where
import Data.Kind (Type)
data X
data Y
class FrozenGen f m where
  type MutableGen f m = g
instance FrozenGen X Y where
  type MutableGen X Y = Int
use :: MutableGen X Y -> Int
use n = n
"####;
    let result_chirho = typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "ProbeChirho.hs",
    );
    let error_chirho = result_chirho
        .err()
        .expect("invalid declaration must be diagnosed")
        .to_string();
    assert!(
        error_chirho.contains("default names no declared associated family"),
        "{error_chirho}"
    );
}

#[test]
fn s2b_bare_result_binder_as_default_use_chirho() {
    let source_chirho = r####"{-# LANGUAGE TypeFamilies, KindSignatures, MultiParamTypeClasses #-}
module ProbeChirho where
import Data.Kind (Type)
data X
data Y
class FrozenGen f m where
  type MutableGen f m = g
instance FrozenGen X Y
use :: MutableGen X Y -> X
use x = x
"####;
    let result_chirho = typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "ProbeChirho.hs",
    );
    let error_chirho = result_chirho
        .err()
        .expect("invalid declaration must be diagnosed")
        .to_string();
    assert!(
        error_chirho.contains("default names no declared associated family"),
        "{error_chirho}"
    );
}

#[test]
fn s3_kinded_class_parameter_as_result_chirho() {
    let source_chirho = r####"{-# LANGUAGE TypeFamilies, KindSignatures, MultiParamTypeClasses #-}
module ProbeChirho where
import Data.Kind (Type)
data X
data Y
class FrozenGen f m where
  type MutableGen f m = (f :: Type)
instance FrozenGen X Y
use :: MutableGen X Y -> X
use x = x
"####;
    let result_chirho = typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "ProbeChirho.hs",
    );
    let error_chirho = result_chirho
        .err()
        .expect("invalid declaration must be diagnosed")
        .to_string();
    assert!(
        error_chirho.contains("associated family result binder requires an injectivity annotation"),
        "{error_chirho}"
    );
}

#[test]
fn s3b_kinded_class_parameter_decl_use_chirho() {
    let source_chirho = r####"{-# LANGUAGE TypeFamilies, KindSignatures, MultiParamTypeClasses #-}
module ProbeChirho where
import Data.Kind (Type)
data X
data Y
class FrozenGen f m where
  type MutableGen f m = (f :: Type)
instance FrozenGen X Y where
  type MutableGen X Y = Int
use :: MutableGen X Y -> Int
use n = n
"####;
    let result_chirho = typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "ProbeChirho.hs",
    );
    let error_chirho = result_chirho
        .err()
        .expect("invalid declaration must be diagnosed")
        .to_string();
    assert!(
        error_chirho.contains("associated family result binder requires an injectivity annotation"),
        "{error_chirho}"
    );
}

#[test]
fn s4_family_head_then_kinded_binder_chirho() {
    let source_chirho = r####"{-# LANGUAGE TypeFamilies, KindSignatures, MultiParamTypeClasses #-}
module ProbeChirho where
import Data.Kind (Type)
data X
data Y
class FrozenGen f m where
  type family MutableGen f m :: Type
  type MutableGen f m = (g :: Type)
instance FrozenGen X Y where
  type MutableGen X Y = Int
use :: MutableGen X Y -> Int
use n = n
"####;
    let result_chirho = typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "ProbeChirho.hs",
    );
    let error_chirho = result_chirho
        .err()
        .expect("invalid declaration must be diagnosed")
        .to_string();
    assert!(
        error_chirho.contains("associated family result binder requires an injectivity annotation"),
        "{error_chirho}"
    );
}

#[test]
fn s6_random_package_form_with_injectivity_chirho() {
    let source_chirho = r####"{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures, MultiParamTypeClasses #-}
module ProbeChirho where
import Data.Kind (Type)
data X
data Y
class FrozenGen f m where
  type MutableGen f m = (g :: Type) | g -> f
instance FrozenGen X Y where
  type MutableGen X Y = Int
use :: MutableGen X Y -> Int
use n = n
"####;
    let result_chirho = typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "ProbeChirho.hs",
    );
    result_chirho.unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn s7_bare_binder_with_injectivity_chirho() {
    let source_chirho = r####"{-# LANGUAGE TypeFamilies, TypeFamilyDependencies, KindSignatures, MultiParamTypeClasses #-}
module ProbeChirho where
import Data.Kind (Type)
data X
data Y
class FrozenGen f m where
  type MutableGen f m = g | g -> f
instance FrozenGen X Y where
  type MutableGen X Y = Int
use :: MutableGen X Y -> Int
use n = n
"####;
    let result_chirho = typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "ProbeChirho.hs",
    );
    result_chirho.unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn s5b_kinded_family_head_then_default_class_param_chirho() {
    let source_chirho = r####"{-# LANGUAGE TypeFamilies, KindSignatures, MultiParamTypeClasses #-}
module ProbeChirho where
import Data.Kind (Type)
data X
data Y
class FrozenGen f m where
  type family MutableGen (f :: Type) (m :: Type) :: Type
  type MutableGen f m = f
instance FrozenGen X Y
use :: MutableGen X Y -> X
use x = x
"####;
    let result_chirho = typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "ProbeChirho.hs",
    );
    result_chirho.unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn associated_default_annotated_variable_chirho() {
    let source_chirho = r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, KindSignatures, FlexibleInstances, MultiParamTypeClasses #-}
module ProbeChirho where
import Data.Kind (Type)
class CChirho (aChirho :: Type) where
  type FChirho aChirho :: Type
  type FChirho (bChirho :: Type) = bChirho
instance CChirho Int
useChirho :: FChirho Int
useChirho = 1
"###;
    let result_chirho = typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "ProbeChirho.hs",
    );
    result_chirho.unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn associated_default_parenthesized_variable_chirho() {
    let source_chirho = r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, KindSignatures, FlexibleInstances, MultiParamTypeClasses #-}
module ProbeChirho where
import Data.Kind (Type)
class CChirho (aChirho :: Type) where
  type FChirho aChirho :: Type
  type FChirho ((bChirho :: Type)) = bChirho
instance CChirho Int
useChirho :: FChirho Int
useChirho = 1
"###;
    let result_chirho = typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "ProbeChirho.hs",
    );
    result_chirho.unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
}

#[test]
fn associated_default_wrong_result_chirho() {
    let source_chirho = r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, KindSignatures, FlexibleInstances, MultiParamTypeClasses #-}
module ProbeChirho where
import Data.Kind (Type)
class CChirho (aChirho :: Type) where
  type FChirho aChirho :: Type
  type FChirho (bChirho :: Type) = bChirho
instance CChirho Int
useChirho :: FChirho Int
useChirho = True
"###;
    let result_chirho = typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "ProbeChirho.hs",
    );
    let error_chirho = result_chirho
        .err()
        .expect("invalid default or consumer must be diagnosed")
        .to_string();
    assert!(error_chirho.contains("Bool"), "{error_chirho}");
}

#[test]
fn associated_default_wrong_kind_chirho() {
    let source_chirho = r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, KindSignatures, FlexibleInstances, MultiParamTypeClasses #-}
module ProbeChirho where
import Data.Kind (Type)
class CChirho (aChirho :: Type) where
  type FChirho aChirho :: Type
  type FChirho (bChirho :: Type -> Type) = Int
instance CChirho Int
useChirho :: FChirho Int
useChirho = 1
"###;
    let result_chirho = typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "ProbeChirho.hs",
    );
    let error_chirho = result_chirho
        .err()
        .expect("invalid default or consumer must be diagnosed")
        .to_string();
    assert!(error_chirho.contains("kind mismatch"), "{error_chirho}");
}

#[test]
fn associated_default_unknown_kind_chirho() {
    let source_chirho = r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, KindSignatures, FlexibleInstances, MultiParamTypeClasses #-}
module ProbeChirho where
import Data.Kind (Type)
class CChirho (aChirho :: Type) where
  type FChirho aChirho :: Type
  type FChirho (bChirho :: MissingKindChirho) = Int
instance CChirho Int
useChirho :: FChirho Int
useChirho = 1
"###;
    let result_chirho = typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "ProbeChirho.hs",
    );
    let error_chirho = result_chirho
        .err()
        .expect("invalid default or consumer must be diagnosed")
        .to_string();
    assert!(error_chirho.contains("MissingKindChirho"), "{error_chirho}");
}

#[test]
fn associated_default_annotated_pattern_chirho() {
    let source_chirho = r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, KindSignatures, FlexibleInstances, MultiParamTypeClasses #-}
module ProbeChirho where
import Data.Kind (Type)
class CChirho (aChirho :: Type) where
  type FChirho aChirho :: Type
  type FChirho ([bChirho] :: Type) = Int
instance CChirho Int
useChirho :: FChirho Int
useChirho = 1
"###;
    let result_chirho = typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "ProbeChirho.hs",
    );
    let error_chirho = result_chirho
        .err()
        .expect("invalid default or consumer must be diagnosed")
        .to_string();
    assert!(error_chirho.contains("distinct variable"), "{error_chirho}");
}

#[test]
fn associated_default_duplicate_variable_chirho() {
    let source_chirho = r###"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, KindSignatures, FlexibleInstances, MultiParamTypeClasses #-}
module ProbeChirho where
import Data.Kind (Type)
class CChirho (aChirho :: Type) (bChirho :: Type) where
  type FChirho aChirho bChirho :: Type
  type FChirho (xChirho :: Type) (xChirho :: Type) = Int
"###;
    let result_chirho = typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "ProbeChirho.hs",
    );
    let error_chirho = result_chirho
        .err()
        .expect("invalid default or consumer must be diagnosed")
        .to_string();
    assert!(error_chirho.contains("distinct variable"), "{error_chirho}");
}

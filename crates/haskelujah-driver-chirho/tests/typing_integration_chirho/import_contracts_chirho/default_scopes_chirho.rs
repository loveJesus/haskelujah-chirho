// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

// Exact sources independently checked by GHC9.14.1 in default-annotations-chirho/scope-chirho.
use haskelujah_driver::typecheck_source_chirho;
use haskelujah_span_chirho::SourceMapChirho;

fn check_chirho(source_chirho: &str, reason_chirho: Option<&str>) {
    let result_chirho = typecheck_source_chirho(
        source_chirho,
        &mut SourceMapChirho::new_chirho(),
        "ProbeChirho.hs",
    );
    if let Some(reason_chirho) = reason_chirho {
        let error_chirho = result_chirho
            .err()
            .expect("invalid default must be rejected")
            .to_string();
        assert!(error_chirho.contains(reason_chirho), "{error_chirho}");
    } else {
        result_chirho.unwrap_or_else(|error_chirho| panic!("{error_chirho}"));
    }
}

#[test]
fn repeated_hidden_kind_chirho() {
    check_chirho(
        r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, KindSignatures, PolyKinds, DataKinds, MultiParamTypeClasses, FlexibleInstances #-}
module ProbeChirho where
import Data.Kind (Type)
import Data.Proxy (Proxy)
class CChirho (aChirho :: jChirho) (bChirho :: kChirho) where
  type FChirho aChirho bChirho :: Type
  type FChirho (aChirho :: zChirho) (bChirho :: zChirho) = Proxy aChirho
"####,
        Some("kind"),
    );
}

#[test]
fn distinct_hidden_kinds_chirho() {
    check_chirho(
        r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, KindSignatures, PolyKinds, DataKinds, MultiParamTypeClasses, FlexibleInstances #-}
module ProbeChirho where
import Data.Kind (Type)
import Data.Proxy (Proxy)
class CChirho (aChirho :: jChirho) (bChirho :: kChirho) where
  type FChirho aChirho bChirho :: Type
  type FChirho (aChirho :: zChirho) (bChirho :: wChirho) = (Proxy aChirho, Proxy bChirho)
"####,
        None,
    );
}

#[test]
fn shared_hidden_kind_chirho() {
    check_chirho(
        r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, KindSignatures, PolyKinds, DataKinds, MultiParamTypeClasses, FlexibleInstances #-}
module ProbeChirho where
import Data.Kind (Type)
import Data.Proxy (Proxy)
class CChirho (aChirho :: kChirho) (bChirho :: kChirho) where
  type FChirho aChirho bChirho :: Type
  type FChirho (aChirho :: zChirho) (bChirho :: zChirho) = (Proxy aChirho, Proxy bChirho)
"####,
        None,
    );
}

#[test]
fn implicit_result_default_chirho() {
    check_chirho(
        r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, KindSignatures, PolyKinds, DataKinds, MultiParamTypeClasses, FlexibleInstances #-}
module ProbeChirho where
import Data.Kind (Type)
import Data.Proxy (Proxy)
class CChirho (aChirho :: Type) where
  type FChirho aChirho
  type FChirho aChirho = Maybe aChirho
instance CChirho Int
useChirho :: FChirho Int
useChirho = Just 3
"####,
        None,
    );
}

#[test]
fn method_fixes_result_kind_chirho() {
    check_chirho(
        r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, KindSignatures, PolyKinds, DataKinds, MultiParamTypeClasses, FlexibleInstances #-}
module ProbeChirho where
import Data.Kind (Type)
import Data.Proxy (Proxy)
class CChirho (aChirho :: Type) where
  type FChirho aChirho
  type FChirho aChirho = Maybe
  methodChirho :: FChirho aChirho Int -> aChirho
"####,
        Some("kind"),
    );
}

#[test]
fn equation_kind_rigid_chirho() {
    check_chirho(
        r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, KindSignatures, PolyKinds, DataKinds, MultiParamTypeClasses, FlexibleInstances #-}
module ProbeChirho where
import Data.Kind (Type)
import Data.Proxy (Proxy)
class CChirho (aChirho :: Type) where
  type FChirho aChirho :: Type
  type FChirho (aChirho :: kChirho) = Int
"####,
        Some("kind"),
    );
}

#[test]
fn equation_kind_rhs_rigid_chirho() {
    check_chirho(
        r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, KindSignatures, PolyKinds, DataKinds, MultiParamTypeClasses, FlexibleInstances #-}
module ProbeChirho where
import Data.Kind (Type)
import Data.Proxy (Proxy)
class CChirho (aChirho :: Type) where
  type FChirho aChirho :: Type
  type FChirho (aChirho :: kChirho) = Proxy (aChirho :: kChirho)
"####,
        Some("kind"),
    );
}

#[test]
fn hidden_kind_specialization_chirho() {
    check_chirho(
        r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, KindSignatures, PolyKinds, DataKinds, MultiParamTypeClasses, FlexibleInstances #-}
module ProbeChirho where
import Data.Kind (Type)
import Data.Proxy (Proxy)
class CChirho (aChirho :: kChirho) where
  type FChirho aChirho :: Type
  type FChirho (aChirho :: Type) = Int
"####,
        Some("kind"),
    );
}

#[test]
fn class_parameter_rhs_chirho() {
    check_chirho(
        r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, KindSignatures, PolyKinds, DataKinds, MultiParamTypeClasses, FlexibleInstances #-}
module ProbeChirho where
import Data.Kind (Type)
import Data.Proxy (Proxy)
class CChirho (aChirho :: Type) (bChirho :: Type) where
  type FChirho aChirho :: Type
  type FChirho xChirho = bChirho
"####,
        Some("type variable not in scope"),
    );
}

#[test]
fn equation_shadows_class_chirho() {
    check_chirho(
        r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, KindSignatures, PolyKinds, DataKinds, MultiParamTypeClasses, FlexibleInstances #-}
module ProbeChirho where
import Data.Kind (Type)
import Data.Proxy (Proxy)
class CChirho (aChirho :: Type) (bChirho :: Type -> Type) where
  type FChirho aChirho :: Type
  type FChirho (bChirho :: Type) = bChirho
instance CChirho Bool Maybe
useChirho :: FChirho Bool
useChirho = True
"####,
        None,
    );
}

#[test]
fn polykind_renamed_equation_chirho() {
    check_chirho(
        r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, KindSignatures, PolyKinds, DataKinds, MultiParamTypeClasses, FlexibleInstances #-}
module ProbeChirho where
import Data.Kind (Type)
import Data.Proxy (Proxy)
class CChirho (aChirho :: kChirho) where
  type FChirho (aChirho :: kChirho) :: Type
  type FChirho (bChirho :: jChirho) = Proxy bChirho
instance CChirho Maybe
useChirho :: FChirho Maybe -> Proxy Maybe
useChirho xChirho = xChirho
"####,
        None,
    );
}

#[test]
fn implicit_lhs_kind_chirho() {
    check_chirho(
        r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, KindSignatures, PolyKinds, DataKinds, MultiParamTypeClasses, FlexibleInstances #-}
module ProbeChirho where
import Data.Kind (Type)
import Data.Proxy (Proxy)
class CChirho (aChirho :: kChirho) where
  type FChirho aChirho :: Type
  type FChirho aChirho = Proxy (aChirho :: kChirho)
instance CChirho Maybe
useChirho :: FChirho Maybe -> Proxy Maybe
useChirho xChirho = xChirho
"####,
        Some("type variable not in scope"),
    );
}

#[test]
fn class_parameter_as_implicit_kind_chirho() {
    check_chirho(
        r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, KindSignatures, PolyKinds, DataKinds, MultiParamTypeClasses, FlexibleInstances #-}
module ProbeChirho where
import Data.Kind (Type)
import Data.Proxy (Proxy)
class CChirho (aChirho :: Type) where
  type FChirho (xChirho :: aChirho) :: [aChirho]
  type FChirho xChirho = ('[] :: [aChirho])
"####,
        Some("type variable not in scope"),
    );
}

#[test]
fn result_kind_implicit_chirho() {
    check_chirho(
        r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, KindSignatures, PolyKinds, DataKinds, MultiParamTypeClasses, FlexibleInstances #-}
module ProbeChirho where
import Data.Kind (Type)
import Data.Proxy (Proxy)
class CChirho (aChirho :: Type) where
  type FChirho aChirho :: [kChirho]
  type FChirho aChirho = ('[] :: [jChirho])
"####,
        Some("type variable not in scope"),
    );
}

#[test]
fn rhs_only_kind_unbound_chirho() {
    check_chirho(
        r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, KindSignatures, PolyKinds, DataKinds, MultiParamTypeClasses, FlexibleInstances #-}
module ProbeChirho where
import Data.Kind (Type)
import Data.Proxy (Proxy)
class CChirho (aChirho :: Type) where
  type FChirho aChirho :: Type
  type FChirho aChirho = Proxy ('[] :: [jChirho])
"####,
        Some("type variable not in scope"),
    );
}

#[test]
fn inferred_class_default_chirho() {
    check_chirho(
        r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, KindSignatures, PolyKinds, DataKinds, MultiParamTypeClasses, FlexibleInstances #-}
module ProbeChirho where
import Data.Kind (Type)
import Data.Proxy (Proxy)
class CChirho aChirho where
  type FChirho aChirho :: Type
  type FChirho aChirho = Maybe aChirho
instance CChirho Int
useChirho :: FChirho Int
useChirho = Just 3
"####,
        Some("kind"),
    );
}

#[test]
fn method_fixes_class_kind_chirho() {
    check_chirho(
        r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE TypeFamilies, KindSignatures, PolyKinds, DataKinds, MultiParamTypeClasses, FlexibleInstances #-}
module ProbeChirho where
import Data.Kind (Type)
import Data.Proxy (Proxy)
class CChirho aChirho where
  type FChirho aChirho :: Type
  type FChirho aChirho = Maybe aChirho
  methodChirho :: aChirho -> aChirho
"####,
        None,
    );
}

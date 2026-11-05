// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Exact frozen product counterexamples. Fixture bytes match the independent
//! f6845284 GHC checkpoint; these tests exercise the actual frontend only.

use super::ordinary_elaboration_chirho::frontend_chirho;

#[test]
fn promoted_constructor_annotation_survives_open_equation_lowering_chirho() {
    frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/P01KProxyChirho.hs"
    ))
    .expect("frozen P01 GHC-positive source checks");
}

#[test]
fn promoted_constructor_synonym_keeps_its_kind_prefix_chirho() {
    frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/P04SynonymKindChirho.hs"
    ))
    .expect("frozen P04 GHC-positive source checks");
}

#[test]
fn written_kind_variables_keep_specified_visibility_chirho() {
    frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/P05ExplicitKindsChirho.hs"
    ))
    .expect("frozen P05 GHC-positive source checks");
}

#[test]
fn proxy_term_and_typeable_instance_use_the_complete_kind_prefix_chirho() {
    frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/TypeablePolykindPrefixChirho.hs"
    ))
    .expect("frozen polymorphic Typeable GHC-positive source checks");
}

#[test]
fn promoted_equation_wrong_result_is_a_real_type_error_chirho() {
    let error_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/N01WrongKProxyResultChirho.hs"
    ))
    .err()
    .expect("wrong result rejected");
    assert!(error_chirho.contains("error[E0200]"), "{error_chirho}");
}

#[test]
fn promoted_same_kind_overlap_is_a_real_coherence_error_chirho() {
    let error_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/N02SameKindOverlapChirho.hs"
    ))
    .err()
    .expect("overlap rejected");
    assert!(error_chirho.contains("overlap"), "{error_chirho}");
}

#[test]
fn qualified_proxy_type_application_skips_its_inferred_kind_chirho() {
    frontend_chirho("{-# LANGUAGE PolyKinds, TypeApplications #-}\nmodule Main where\nimport qualified Data.Proxy as PChirho\nvalueChirho :: PChirho.Proxy Maybe\nvalueChirho = PChirho.Proxy @Maybe\nmain = pure ()\n").expect("qualified Proxy applies its specified type parameter");
}

#[test]
fn inferred_constructor_kind_is_not_an_extra_visible_type_argument_chirho() {
    let error_chirho = frontend_chirho("{-# LANGUAGE PolyKinds, TypeApplications #-}\nmodule Main where\nimport Data.Proxy (Proxy(..))\nvalueChirho = Proxy @Maybe @Bool\nmain = pure ()\n").err().expect("inferred kind binder cannot be explicitly applied");
    assert!(
        error_chirho.contains("no specified constructor type binder"),
        "{error_chirho}"
    );
}

#[test]
fn source_constructor_inferred_kinds_stay_invisible_chirho() {
    frontend_chirho("{-# LANGUAGE PolyKinds, KindSignatures, TypeApplications #-}\nmodule Main where\ndata PhantomChirho (a :: k) = MkChirho\nvalueChirho :: PhantomChirho Maybe\nvalueChirho = MkChirho @Maybe\nmain = pure ()\n").expect("source constructor skips its inferred kind parameter");
}

#[test]
fn constructor_visible_type_application_checks_parameter_kinds_chirho() {
    let error_chirho = frontend_chirho("{-# LANGUAGE TypeApplications #-}\nmodule Main where\nvalueChirho = Nothing @Maybe\nmain = pure ()\n").err().expect("Maybe cannot fill a Type constructor parameter");
    assert!(error_chirho.contains("error[E0300]"), "{error_chirho}");
}

#[test]
fn deferred_source_class_kind_substitution_remains_linked_chirho() {
    frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/PC1DeferredKindSubstitutionChirho.hs"
    ))
    .expect("GHC-positive deferred predicate uses the substituted kind");
}

#[test]
fn source_class_instance_entailment_keeps_the_full_prefix_chirho() {
    frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/PC2InstanceEntailmentPrefixChirho.hs"
    ))
    .expect("GHC-positive source class givens remain in scope");
}

#[test]
fn source_class_method_uses_the_instance_kind_argument_chirho() {
    frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/PC3MethodKindOwnershipChirho.hs"
    ))
    .expect("GHC-positive method specializes the class kind");
}

#[test]
fn direct_source_class_twin_remains_accepted_chirho() {
    frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/PC1DirectPassingTwinChirho.hs"
    ))
    .expect("GHC-positive direct twin stays accepted");
}

#[test]
fn source_class_method_cannot_use_an_incompatible_kind_chirho() {
    let source_chirho = include_str!("kind_repair_fixtures_chirho/PC3MethodKindOwnershipChirho.hs")
        .replace(
            "instance NameChirho Maybe where",
            "instance NameChirho Bool where",
        )
        .replace(
            "TagChirho :: TagChirho Maybe",
            "TagChirho :: TagChirho Bool",
        );
    let error_chirho = frontend_chirho(&source_chirho)
        .err()
        .expect("unary method cannot serve a Type-kinded instance");
    assert!(error_chirho.contains("error[E0200]"), "{error_chirho}");
}

#[test]
fn source_class_deferred_predicate_requires_the_matching_instance_chirho() {
    let source_chirho =
        include_str!("kind_repair_fixtures_chirho/PC1DeferredKindSubstitutionChirho.hs")
            .replace("TagChirho :: TagChirho Maybe", "TagChirho :: TagChirho []");
    let error_chirho = frontend_chirho(&source_chirho)
        .err()
        .expect("same-kind list has no source class instance");
    assert!(error_chirho.contains("error[E0204]"), "{error_chirho}");
}

#[test]
fn associated_default_preserves_shared_hidden_kinds_chirho() {
    frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/associated-chirho/HiddenDefaultChirho.hs"
    ))
    .expect("associated default uses the class's complete telescope");
}

#[test]
fn associated_default_renaming_retains_reordered_and_family_local_slots_chirho() {
    frontend_chirho(include_str!("kind_repair_fixtures_chirho/associated-chirho/MonokindedReorderedDefaultChirho.hs")).expect("producer correspondence retains subset/reordered class parameters and family-only variables");
}

#[test]
fn explicit_associated_equation_overrides_the_checked_default_chirho() {
    frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/associated-chirho/MonokindedOverrideDefaultChirho.hs"
    ))
    .expect("explicit canonical family equation wins over its class default");
}

#[test]
fn associated_default_requires_the_checked_result_chirho() {
    let error_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/associated-chirho/MonokindedWrongDefaultResultChirho.hs"
    ))
    .err()
    .expect("Bool cannot stand in for the default Maybe Int");
    assert!(error_chirho.contains("error[E0200]"), "{error_chirho}");
}

#[test]
fn an_associated_default_cannot_reduce_without_its_class_instance_chirho() {
    let source_chirho = include_str!(
        "kind_repair_fixtures_chirho/associated-chirho/MonokindedWrongDefaultResultChirho.hs"
    )
    .replace("instance CChirho Int\n", "")
    .replace("valueChirho = True", "valueChirho = Just (1 :: Int)");
    let error_chirho = frontend_chirho(&source_chirho)
        .err()
        .expect("a default is not an abstract synonym equation");
    assert!(error_chirho.contains("error[E0200]"), "{error_chirho}");
}

#[test]
fn a_polykinded_default_cannot_specialize_the_class_header_chirho() {
    let error_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/associated-chirho/PolyKindSpecializingDefaultChirho.hs"
    ))
    .err()
    .expect("a default cannot constrain an already generalized hidden kind");
    assert!(error_chirho.contains("error[E0300]"), "{error_chirho}");
}

#[test]
fn qualified_proxy_keeps_its_producer_beside_a_local_homonym_chirho() {
    frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/qualification-chirho/QualifiedProxyHomonymChirho.hs"
    ))
    .expect("actual selected builtin producer retains Proxy's complete kind seed");
}

#[test]
fn qualified_proxy_never_borrows_a_source_namesakes_origin_chirho() {
    let error_chirho = frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/qualification-chirho/QualifiedProxyWrongOriginChirho.hs"
    ))
    .err()
    .expect("source and builtin Proxy remain different nominal types");
    assert!(error_chirho.contains("error[E0200]"), "{error_chirho}");
}

// Immutable GHC9.14.1 contract: T13142; SHA256 7fa75a4a79f14990e4afbb63f5f8a93f1e1ef871456967245138b65178dba235.
#[test]
fn instance_nat_binder_keeps_its_checked_kind_chirho() {
    frontend_chirho(
        r####"{-# LANGUAGE BangPatterns #-}
{-# LANGUAGE DataKinds #-}
{-# LANGUAGE DefaultSignatures #-}
{-# LANGUAGE DeriveFunctor #-}
{-# LANGUAGE DeriveGeneric #-}
{-# LANGUAGE EmptyCase #-}
{-# LANGUAGE FlexibleContexts #-}
{-# LANGUAGE FlexibleInstances #-}
{-# LANGUAGE KindSignatures #-}
{-# LANGUAGE MagicHash #-}
{-# LANGUAGE MultiParamTypeClasses #-}
{-# LANGUAGE RankNTypes #-}
{-# LANGUAGE ScopedTypeVariables #-}
{-# LANGUAGE TypeFamilies #-}
{-# LANGUAGE TypeOperators #-}
{-# LANGUAGE UnboxedTuples #-}
{-# LANGUAGE UndecidableInstances #-}

module T13142 () where

import Data.Foldable (foldl')
import Data.Proxy
import Data.Word
import Foreign.Storable (Storable, sizeOf)
import GHC.Generics
import qualified Data.Kind as K
import GHC.TypeLits

newtype Type = VarT Name
  deriving Generic

data Name = Name OccName NameFlavour
  deriving Generic

data NameFlavour
   = NameS
   | NameQ ModName
   | NameU !Int
   | NameL !Int
   | NameG PkgName ModName
   deriving Generic

newtype ModName = ModName String
  deriving Generic

newtype PkgName = PkgName String
  deriving Generic

newtype OccName = OccName String
  deriving Generic

instance Store Type
instance Store Name
instance Store NameFlavour
instance Store ModName
instance Store OccName
instance Store PkgName

instance Store Char where
  {-# INLINE size #-}
  size = sizeStorableTy "Foreign.Storable.Storable GHC.Types.Char"

instance Store Int where
  {-# INLINE size #-}
  size = undefined

instance Store a => Store [a] where
    size = sizeSequence
    {-# INLINE size #-}

sizeSequence :: forall a. Store a => Size [a]
sizeSequence = VarSize $ \t ->
    case size :: Size a of
        ConstSize n -> n * (length t) + sizeOf (undefined :: Int)
        VarSize f -> foldl' (\acc x -> acc + f x) (sizeOf (undefined :: Int)) t
{-# INLINE sizeSequence #-}

class Store a where
    size :: Size a

    default size :: (Generic a, GStoreSize (Rep a)) => Size a
    size = genericSize
    {-# INLINE size #-}

data Size a
    = VarSize (a -> Int)
    | ConstSize !Int

getSizeWith :: Size a -> a -> Int
getSizeWith (VarSize f) x = f x
getSizeWith (ConstSize n) _ = n
{-# INLINE getSizeWith #-}

contramapSize :: (a -> b) -> Size b -> Size a
contramapSize f (VarSize g) = VarSize (g . f)
contramapSize _ (ConstSize n) = ConstSize n
{-# INLINE contramapSize #-}

combineSizeWith :: forall a b c. (c -> a) -> (c -> b) -> Size a -> Size b -> Size c
combineSizeWith toA toB sizeA sizeB =
    case (sizeA, sizeB) of
        (VarSize f, VarSize g) -> VarSize (\x -> f (toA x) + g (toB x))
        (VarSize f, ConstSize m) -> VarSize (\x -> f (toA x) + m)
        (ConstSize n, VarSize g) -> VarSize (\x -> n + g (toB x))
        (ConstSize n, ConstSize m) -> ConstSize (n + m)
{-# INLINE combineSizeWith #-}

sizeStorableTy :: forall a. Storable a => String -> Size a
sizeStorableTy ty = ConstSize (sizeOf (error msg :: a))
  where
    msg = "In Data.Store.storableSize: " ++ ty ++ "'s sizeOf evaluated its argument."
{-# INLINE sizeStorableTy #-}

genericSize :: (Generic a, GStoreSize (Rep a)) => Size a
genericSize = contramapSize from gsize
{-# INLINE genericSize #-}

type family SumArity (a :: K.Type -> K.Type) :: Nat where
    SumArity (C1 c a) = 1
    SumArity (x :+: y) = SumArity x + SumArity y

class GStoreSize f where gsize :: Size (f a)

instance GStoreSize f => GStoreSize (M1 i c f) where
    gsize = contramapSize unM1 gsize
    {-# INLINE gsize #-}

instance Store a => GStoreSize (K1 i a) where
    gsize = contramapSize unK1 size
    {-# INLINE gsize #-}

instance GStoreSize U1 where
    gsize = ConstSize 0
    {-# INLINE gsize #-}

instance GStoreSize V1 where
    gsize = ConstSize 0
    {-# INLINE gsize #-}

instance (GStoreSize a, GStoreSize b) => GStoreSize (a :*: b) where
    gsize = combineSizeWith (\(x :*: _) -> x) (\(_ :*: y) -> y) gsize gsize
    {-# INLINE gsize #-}

instance (SumArity (a :+: b) <= 255, GStoreSizeSum 0 (a :+: b))
         => GStoreSize (a :+: b) where
    gsize = VarSize $ \x -> sizeOf (undefined :: Word8) + gsizeSum x (Proxy :: Proxy 0)
    {-# INLINE gsize #-}

class KnownNat n => GStoreSizeSum (n :: Nat) (f :: K.Type -> K.Type) where
    gsizeSum :: f a -> Proxy n -> Int

instance (GStoreSizeSum n a, GStoreSizeSum (n + SumArity a) b, KnownNat n)
         => GStoreSizeSum n (a :+: b) where
    gsizeSum (L1 l) _ = gsizeSum l (Proxy :: Proxy n)
    gsizeSum (R1 r) _ = gsizeSum r (Proxy :: Proxy (n + SumArity a))
    {-# INLINE gsizeSum #-}

instance (GStoreSize a, KnownNat n) => GStoreSizeSum n (C1 c a) where
    gsizeSum x _ = getSizeWith gsize x
    {-# INLINE gsizeSum #-}
"####,
    )
    .expect("independently measured GHC-positive instance source checks");
}

// Immutable GHC9.14.1 contract: T8392; SHA256 b9c65300f5c2efca57c1de1eb8d28470e33652830d11c7cee9d6a2de69e3df68.
#[test]
fn instance_context_only_binder_keeps_its_checked_kind_chirho() {
    frontend_chirho(r####"{-# LANGUAGE FlexibleInstances, MultiParamTypeClasses, ScopedTypeVariables, TypeFamilies, UndecidableInstances, AllowAmbiguousTypes #-}

module T8392 where

class Fun f a b where
    fun :: f -> a -> b

instance (b ~ Int, a ~ Int) => Fun F a b
    where fun _ = (+1)

data F = F

data Compose a b = Compose a b

-- ghc-7.6 version
instance (Fun f b c, Fun g a b) => Fun (Compose f g) a c where
    fun (Compose f g) a = fun f (fun g a :: b)"####).expect("independently measured GHC-positive instance source checks");
}

// Immutable GHC9.14.1 contract: tc153; SHA256 630ea215a38c888b9b6f89e291a00a95d7936b4de1405524a1562ef3da643255.
#[test]
fn unscoped_instance_body_annotation_quantifies_independently_chirho() {
    frontend_chirho(
        r####"{-# LANGUAGE Haskell2010 #-}
-- No ScopedTypeVariables, so (v::a) means (v:: forall a.a)

module ShouldCompile where

data T a = T a

instance Eq (T a) where
  (==) x y = let v :: a
                 v = undefined
             in
                v

"####,
    )
    .expect("independently measured GHC-positive instance source checks");
}

// Immutable GHC9.14.1 contract: InstanceScopedResultWrongChirho; SHA256 bcf9796375549166bce3a132e62046505b9df5b885abbcb3dc861d3377480950.
#[test]
fn scoped_instance_result_cannot_choose_a_rigid_owner_chirho() {
    let error_chirho = frontend_chirho(r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE Haskell2010, ScopedTypeVariables #-}
-- No ScopedTypeVariables, so (v::a) means (v:: forall a.a)

module ShouldCompile where

data T a = T a

instance Eq (T a) where
  (==) x y = let v :: a
                 v = undefined
             in
                v

"####).err().expect("rigid instance result rejects");
    assert!(error_chirho.contains("error[E0200]"), "{error_chirho}");
}

// Immutable GHC9.14.1 contract: InstanceScopedBodyPositiveChirho; SHA256 d713f6b232d3856837e3e8def3199c0992507128c0c163b19d345ba8c20b2440.
#[test]
fn scoped_instance_body_retains_its_actual_owner_chirho() {
    frontend_chirho(r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE Haskell2010, ScopedTypeVariables #-}
-- No ScopedTypeVariables, so (v::a) means (v:: forall a.a)

module ShouldCompile where

data T a = T a

instance Eq (T a) where
  (==) x y = let v :: a
                 v = undefined
             in
                case v of { _ -> True }

"####).expect("independently measured GHC-positive instance source checks");
}

// Exact independently measured GHC source SHA256 b9fdab70c95a95386fb45ce861dd7bd32eaa92980cd4dbb7dee3c1f119558f9c.
#[test]
fn class_default_unscoped_body_positive_aligned_chirho() {
    frontend_chirho(r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE Haskell2010 #-}
module ProbeChirho where
class ClassChirho aChirho where
  methodChirho :: aChirho -> Bool
  methodChirho _ = let valueChirho :: aChirho
                       valueChirho = undefined
                  in valueChirho
"####).expect("measured GHC-positive method/layout control checks");
}

// Exact independently measured GHC source SHA256 086ed1a782c2ea870db567eb5358b687f726e67f4fb74a0df7c5b48bec9d3a73.
#[test]
fn class_default_scoped_body_wrong_aligned_chirho() {
    let error_chirho = frontend_chirho(r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE Haskell2010, ScopedTypeVariables #-}
module ProbeChirho where
class ClassChirho aChirho where
  methodChirho :: aChirho -> Bool
  methodChirho _ = let valueChirho :: aChirho
                       valueChirho = undefined
                  in valueChirho
"####).err().expect("measured GHC-negative method/layout control rejects");
    assert!(error_chirho.contains("error[E0200]"), "{error_chirho}");
}

// Exact independently measured GHC source SHA256 b32acfef1df929f105b37843840524586aca3458ed77192286cb75778fada177.
#[test]
fn class_default_scoped_body_positive_aligned_chirho() {
    frontend_chirho(r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE Haskell2010, ScopedTypeVariables #-}
module ProbeChirho where
class ClassChirho aChirho where
  methodChirho :: aChirho -> Bool
  methodChirho _ = let valueChirho :: aChirho
                       valueChirho = undefined
                  in case valueChirho of { _ -> True }
"####).expect("measured GHC-positive method/layout control checks");
}

// Exact independently measured GHC source SHA256 e4ec14fb4a15cb29a1d94cc363ea288d2cf38decf6c195a0d508e7dfb707caf4.
#[test]
fn associated_head_restored_after_body_positive_aligned_chirho() {
    frontend_chirho(r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE Haskell2010, TypeFamilies #-}
module ProbeChirho where
data WrapChirho aChirho = WrapChirho aChirho
class ClassChirho tChirho where
  type AssocChirho tChirho
  methodChirho :: tChirho -> Bool
instance ClassChirho (WrapChirho aChirho) where
  type AssocChirho (WrapChirho aChirho) = aChirho
  methodChirho _ = let valueChirho :: aChirho
                       valueChirho = undefined
                  in valueChirho
"####).expect("measured GHC-positive method/layout control checks");
}

// Exact independently measured GHC source SHA256 2f316cb61d0e237abcce5dda85e8981a0b24c25a77c677bc6e10e9d1e3ef1c0b.
#[test]
fn class_default_unscoped_body_positive_explicit_chirho() {
    frontend_chirho(r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE Haskell2010 #-}
module ProbeChirho where
class ClassChirho aChirho where
  methodChirho :: aChirho -> Bool
  methodChirho _ = let { valueChirho :: aChirho; valueChirho = undefined } in valueChirho
"####).expect("measured GHC-positive method/layout control checks");
}

// Exact independently measured GHC source SHA256 9326b820c72eecc1dc297a20bbaedb3f2dd4dd2ca9e2861522d28c371c6aa97a.
#[test]
fn class_default_scoped_body_wrong_explicit_chirho() {
    let error_chirho = frontend_chirho(r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE Haskell2010, ScopedTypeVariables #-}
module ProbeChirho where
class ClassChirho aChirho where
  methodChirho :: aChirho -> Bool
  methodChirho _ = let { valueChirho :: aChirho; valueChirho = undefined } in valueChirho
"####).err().expect("measured GHC-negative method/layout control rejects");
    assert!(error_chirho.contains("error[E0200]"), "{error_chirho}");
}

// Exact independently measured GHC source SHA256 008357bcfb03e2fe53cfe61ce80603bd5b62ee6b730ec27ac19b5b8ec26e2363.
#[test]
fn class_default_scoped_body_positive_explicit_chirho() {
    frontend_chirho(r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE Haskell2010, ScopedTypeVariables #-}
module ProbeChirho where
class ClassChirho aChirho where
  methodChirho :: aChirho -> Bool
  methodChirho _ = let { valueChirho :: aChirho; valueChirho = undefined } in case valueChirho of { _ -> True }
"####).expect("measured GHC-positive method/layout control checks");
}

// Exact independently measured GHC source SHA256 5cc29fc7c9d6b5f9ea3ba01df561d37c5dcbcd11798653c5d1b4fd3b26b4a2cd.
#[test]
fn associated_head_restored_after_body_positive_explicit_chirho() {
    frontend_chirho(r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE Haskell2010, TypeFamilies #-}
module ProbeChirho where
data WrapChirho aChirho = WrapChirho aChirho
class ClassChirho tChirho where
  type AssocChirho tChirho
  methodChirho :: tChirho -> Bool
instance ClassChirho (WrapChirho aChirho) where
  type AssocChirho (WrapChirho aChirho) = aChirho
  methodChirho _ = let { valueChirho :: aChirho; valueChirho = undefined } in valueChirho
"####).expect("measured GHC-positive method/layout control checks");
}

// Exact independently measured GHC source SHA256 3f4ff3a154c0fb32406d2a0ab36651bbec950822a011279310c74b5dde57cd9f.
#[test]
fn nested_implicit_explicit_let_positive_chirho() {
    frontend_chirho(r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module ProbeChirho where
valueChirho :: Int
valueChirho = let outerChirho = let { innerChirho = 1 :: Int } in innerChirho in outerChirho
"####).expect("measured GHC-positive method/layout control checks");
}

// Exact independently measured GHC source SHA256 d6478f8a1f11805cf279ff621b513619a8646b441cf522528c4a40b64fda2404.
#[test]
fn implicit_let_explicit_case_positive_chirho() {
    frontend_chirho(r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module ProbeChirho where
valueChirho :: Int
valueChirho = let outerChirho = case True of { True -> 1; False -> 0 } in outerChirho
"####).expect("measured GHC-positive method/layout control checks");
}

// Exact independently measured GHC source SHA256 7d4d433cfc170a8b8ad3342a63f6696913eac28546b7af1ea51e74d33cbd794c.
#[test]
fn explicit_let_missing_in_wrong_chirho() {
    let error_chirho = frontend_chirho(r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module ProbeChirho where
valueChirho = let { innerChirho = 1 :: Int }
"####).err().expect("measured GHC-negative method/layout control rejects");
    assert!(error_chirho.contains("error[E0002]"), "{error_chirho}");
}

// Exact independently measured GHC source SHA256 2ca22d96cf9b53cd40982aac56a6f039ccf694bfb4a608cfdfd5a963f8a2d6a3.
#[test]
fn instance_signature_head_positive_chirho() {
    frontend_chirho(r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE Haskell2010, InstanceSigs #-}
module ProbeChirho where
data WrapChirho aChirho = WrapChirho aChirho
class ClassChirho tChirho where
  methodChirho :: tChirho -> Bool
instance ClassChirho (WrapChirho aChirho) where
  methodChirho :: WrapChirho aChirho -> Bool
  methodChirho _ = True
"####).expect("measured GHC-positive method/layout control checks");
}

// Exact independently measured GHC source SHA256 18aeedec491a1acc2eb95b6f8941f506c3c20a16c5dce9f708759222599aa4d1.
#[test]
fn associated_head_wrong_kind_chirho() {
    let error_chirho = frontend_chirho(r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE Haskell2010, TypeFamilies #-}
module ProbeChirho where
data WrapChirho aChirho = WrapChirho aChirho
class ClassChirho tChirho where
  type AssocChirho tChirho
  methodChirho :: tChirho -> Bool
instance ClassChirho (WrapChirho aChirho) where
  type AssocChirho (WrapChirho aChirho) = Maybe
  methodChirho _ = True
"####).err().expect("measured GHC-negative method/layout control rejects");
    assert!(error_chirho.contains("error[E0300]"), "{error_chirho}");
}

// Independent GHC9.14.1 hidden-prefix/shadowing control SHA256 0643277e00bb6e9033d7772af5c2f635b60f1b4c7aa176ab0bfccc0fc319b29e.
#[test]
fn class_hidden_kind_default_positive_chirho() {
    frontend_chirho(r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, ScopedTypeVariables, KindSignatures, TypeApplications, AllowAmbiguousTypes #-}
module ProbeChirho where
import Data.Proxy (Proxy(..))
class ClassChirho (aChirho :: kChirho) where
  methodChirho :: Proxy aChirho -> Proxy kChirho
  methodChirho _ = Proxy @kChirho
"####).expect("GHC-positive default keeps full prefix and lexical shadowing");
}

// Independent GHC9.14.1 hidden-prefix/shadowing control SHA256 3c4ae3a5bc72d08e2bb5839b6a9d34fb2da4150328b57bf30c6fc5c44b8e4d04.
#[test]
fn class_hidden_kind_default_wrong_chirho() {
    let error_chirho = frontend_chirho(r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PolyKinds, ScopedTypeVariables, KindSignatures, TypeApplications, AllowAmbiguousTypes #-}
module ProbeChirho where
import Data.Proxy (Proxy(..))
class ClassChirho (aChirho :: kChirho) where
  methodChirho :: Proxy aChirho -> Proxy kChirho
  methodChirho _ = Proxy @Bool
"####).err().expect("a rigid hidden kind cannot choose Bool");
    assert!(error_chirho.contains("error[E0200]"), "{error_chirho}");
}

// Independent GHC9.14.1 hidden-prefix/shadowing control SHA256 547952e7ad2a250018000a8c81a392fe18d4310e3d1d3abf1c602b41448afb75.
#[test]
fn class_method_forall_shadow_positive_chirho() {
    frontend_chirho(r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE ScopedTypeVariables, RankNTypes, AllowAmbiguousTypes #-}
module ProbeChirho where
class ClassChirho aChirho where
  methodChirho :: forall aChirho. aChirho -> aChirho
  methodChirho (valueChirho :: aChirho) = valueChirho
"####).expect("GHC-positive default keeps full prefix and lexical shadowing");
}

// Independent GHC9.14.1 hidden-prefix/shadowing control SHA256 dbcb55c420ca17bd76c5debba3145dccacf661d171ff4fa78b7ca4208d489c8b.
#[test]
fn class_nested_annotation_shadow_positive_chirho() {
    frontend_chirho(r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE ScopedTypeVariables, RankNTypes #-}
module ProbeChirho where
class ClassChirho aChirho where
  methodChirho :: aChirho -> Bool
  methodChirho _ = let { valueChirho :: forall aChirho. aChirho; valueChirho = undefined } in valueChirho
"####).expect("GHC-positive default keeps full prefix and lexical shadowing");
}

// GHC-measured scoped versus structural deferral SHA256 c2a20f3ef7b5733602ffe101c0f705c7cc4d60abb0327b3bfce81a22a78f0997.
#[test]
fn class_default_scoped_wrong_deferred_chirho() {
    frontend_chirho(r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# OPTIONS_GHC -fdefer-type-errors #-}
{-# LANGUAGE Haskell2010, ScopedTypeVariables #-}
module ProbeChirho where
class ClassChirho aChirho where
  methodChirho :: aChirho -> Bool
  methodChirho _ = let { valueChirho :: aChirho; valueChirho = undefined } in valueChirho
"####).expect("GHC defers this scoped body mismatch");
}

// GHC-measured scoped versus structural deferral SHA256 637a7dc8cd57d130e8e71380cc5e1aeb3bf55f62ffd1620e2334a3e1053ec46e.
#[test]
fn instance_scoped_wrong_deferred_chirho() {
    frontend_chirho(r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# OPTIONS_GHC -fdefer-type-errors #-}
{-# LANGUAGE Haskell2010, ScopedTypeVariables #-}
-- No ScopedTypeVariables, so (v::a) means (v:: forall a.a)

module ShouldCompile where

data T a = T a

instance Eq (T a) where
  (==) x y = let v :: a
                 v = undefined
             in
                v

"####).expect("GHC defers this scoped body mismatch");
}

// GHC-measured scoped versus structural deferral SHA256 ce8a2e92e79c7c9782c17795778a616cda9fde2a7245cec5a81e36cfcddcba81.
#[test]
fn explicit_let_missing_in_deferred_wrong_chirho() {
    let error_chirho = frontend_chirho(r####"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# OPTIONS_GHC -fdefer-type-errors #-}
module ProbeChirho where
valueChirho = let { innerChirho = 1 :: Int }
"####).err().expect("structural syntax remains fatal under deferral");
    assert!(error_chirho.contains("error[E0002]"), "{error_chirho}");
}

#[test]
fn pattern_syntax_corpus_t11524_chirho() {
    frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/syntax_chirho/T11524.hs"
    ))
    .expect("fresh GHC-positive exact corpus source checks");
}

#[test]
fn pattern_syntax_corpus_t18005_chirho() {
    frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/syntax_chirho/T18005.hs"
    ))
    .expect("fresh GHC-positive exact corpus source checks");
}

#[test]
fn pattern_syntax_corpus_t21501_chirho() {
    frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/syntax_chirho/T21501.hs"
    ))
    .expect("fresh GHC-positive exact corpus source checks");
}

#[test]
fn pattern_syntax_corpus_t7169_chirho() {
    frontend_chirho(include_str!(
        "kind_repair_fixtures_chirho/syntax_chirho/T7169.hs"
    ))
    .expect("fresh GHC-positive exact corpus source checks");
}

#[test]
fn pattern_signature_patternnoscopedchirho_chirho() {
    frontend_chirho(r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PatternSynonyms, ViewPatterns, ExplicitForAll, KindSignatures, PolyKinds, TypeApplications #-}
module PatternNoScopedChirho where
import Data.Kind (Type)
import Data.Proxy (Proxy(..))
pattern PChirho :: forall (f_chirho :: Type -> Type). Proxy f_chirho
pattern PChirho <- (id @(Proxy f_chirho) -> Proxy)
"#).expect("fresh GHC pattern-signature scoped positive checks");
}

#[test]
fn pattern_signature_patternscopedchirho_chirho() {
    frontend_chirho(r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PatternSynonyms, ViewPatterns, ExplicitForAll, KindSignatures, PolyKinds, TypeApplications, ScopedTypeVariables #-}
module PatternScopedChirho where
import Data.Kind (Type)
import Data.Proxy (Proxy(..))
pattern PChirho :: forall (f_chirho :: Type -> Type). Proxy f_chirho
pattern PChirho <- (id @(Proxy f_chirho) -> Proxy)
"#).expect("fresh GHC pattern-signature scoped positive checks");
}

#[test]
fn pattern_signature_patternabstractionscopedchirho_chirho() {
    frontend_chirho(r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PatternSynonyms, ViewPatterns, ExplicitForAll, KindSignatures, PolyKinds, TypeApplications, TypeAbstractions #-}
module PatternAbstractionScopedChirho where
import Data.Kind (Type)
import Data.Proxy (Proxy(..))
pattern PChirho :: forall (f_chirho :: Type -> Type). Proxy f_chirho
pattern PChirho <- (id @(Proxy f_chirho) -> Proxy)
"#).expect("fresh GHC pattern-signature scoped positive checks");
}

#[test]
fn pattern_signature_patternwrongkindchirho_chirho() {
    let error_chirho = frontend_chirho(r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE PatternSynonyms, ViewPatterns, ExplicitForAll, KindSignatures, PolyKinds, TypeApplications, ScopedTypeVariables #-}
module PatternWrongKindChirho where
import Data.Kind (Type)
import Data.Proxy (Proxy(..))
pattern PChirho :: forall (f_chirho :: Type -> Type). Proxy f_chirho
pattern PChirho <- (id @(Proxy @Type f_chirho) -> Proxy)
"#).err().expect("fresh GHC wrong-kind negative rejects");
    assert!(error_chirho.contains("error[E0300]"), "{error_chirho}");
}

#[test]
fn representation_producer_typedrepheaderchirho_chirho() {
    frontend_chirho(r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, PolyKinds, KindSignatures, EmptyDataDecls #-}
module TypedRepHeaderChirho where
import GHC.Exts
data PChirho (a_chirho :: TYPE 'IntRep) = PChirho
"#).expect("fresh GHC-positive representation producer checks");
}

#[test]
fn representation_producer_boxedrepwrongchirho_chirho() {
    let error_chirho = frontend_chirho(r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, PolyKinds, KindSignatures, EmptyDataDecls #-}
module BoxedRepWrongChirho where
import GHC.Exts
data PChirho (a_chirho :: TYPE ('BoxedRep 'True)) = PChirho
"#).err().expect("fresh GHC-negative representation misuse rejects");
    assert!(error_chirho.contains("error[E0300]"), "{error_chirho}");
}

#[test]
fn representation_producer_vecrepheaderchirho_chirho() {
    frontend_chirho(r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, PolyKinds, KindSignatures, EmptyDataDecls #-}
module VecRepHeaderChirho where
import GHC.Exts
data PChirho (a_chirho :: TYPE ('VecRep 'Vec4 'Int32ElemRep)) = PChirho
"#).expect("fresh GHC-positive representation producer checks");
}

#[test]
fn representation_producer_vecrepwrongchirho_chirho() {
    let error_chirho = frontend_chirho(r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, PolyKinds, KindSignatures, EmptyDataDecls #-}
module VecRepWrongChirho where
import GHC.Exts
data PChirho (a_chirho :: TYPE ('VecRep 'Vec4 'Lifted)) = PChirho
"#).err().expect("fresh GHC-negative representation misuse rejects");
    assert!(error_chirho.contains("error[E0300]"), "{error_chirho}");
}

#[test]
fn representation_producer_qualifiedrepheaderchirho_chirho() {
    frontend_chirho(r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, PolyKinds, KindSignatures, EmptyDataDecls #-}
module QualifiedRepHeaderChirho where
import qualified GHC.Types as RChirho
data PChirho (a_chirho :: RChirho.TYPE 'RChirho.IntRep) = PChirho
"#).expect("fresh GHC-positive representation producer checks");
}

#[test]
fn representation_producer_sourcerephomonymchirho_chirho() {
    frontend_chirho(r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, PolyKinds, KindSignatures, EmptyDataDecls #-}
module SourceRepHomonymChirho where
data RuntimeRep = RepChirho
data TYPE (r_chirho :: RuntimeRep)
data PChirho (a_chirho :: TYPE 'RepChirho) = PChirho
"#).expect("fresh GHC-positive representation producer checks");
}

#[test]
fn representation_producer_zerobitaliaschirho_chirho() {
    frontend_chirho(r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, PolyKinds, KindSignatures, EmptyDataDecls #-}
module ZeroBitAliasChirho where
import GHC.Exts
import Data.Proxy
valueChirho :: Proxy ZeroBitRep -> Proxy ('TupleRep ('[] :: [RuntimeRep]))
valueChirho = id
"#).expect("fresh GHC-positive representation producer checks");
}

#[test]
fn representation_producer_zerobitaliaswrongchirho_chirho() {
    let error_chirho = frontend_chirho(r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, PolyKinds, KindSignatures, EmptyDataDecls #-}
module ZeroBitAliasWrongChirho where
import GHC.Exts
import Data.Proxy
valueChirho :: Proxy ZeroBitRep -> Proxy ('TupleRep '['IntRep])
valueChirho = id
"#).err().expect("fresh GHC58481 unspaced syntax failure rejects");
    assert!(error_chirho.contains("error[E0002]"), "{error_chirho}");
}

#[test]
fn representation_producer_zerobitaliasspacedwrongchirho_chirho() {
    let error_chirho = frontend_chirho(r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, PolyKinds, KindSignatures, EmptyDataDecls #-}
module ZeroBitAliasSpacedWrongChirho where
import GHC.Exts
import Data.Proxy
valueChirho :: Proxy ZeroBitRep -> Proxy ('TupleRep '[ 'IntRep ])
valueChirho = id
"#).err().expect("fresh GHC type mismatch rejects");
    assert!(
        !error_chirho.contains("error[E0002]"),
        "valid syntax reaches the solver: {error_chirho}"
    );
    assert!(
        error_chirho.contains("error[E0200]") || error_chirho.contains("error[E0205]"),
        "{error_chirho}"
    );
}

#[test]
fn representation_producer_onerepaliaschirho_chirho() {
    frontend_chirho(r#"-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE DataKinds, PolyKinds, KindSignatures, EmptyDataDecls #-}
module OneRepAliasChirho where
import GHC.Exts
import Data.Proxy
type OneRepChirho = 'TupleRep '[ 'IntRep ]
valueChirho :: Proxy OneRepChirho -> Proxy ('TupleRep '[ 'IntRep ])
valueChirho = id
"#).expect("fresh GHC-positive nonempty promoted alias checks");
}

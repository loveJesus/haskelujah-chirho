// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Contiguous standard-seed segment; order is owned by the parent dispatcher.

use crate::ty_chirho::{TyChirho, TyVarChirho};

use super::super::{ClassEnvChirho, PredChirho};

impl ClassEnvChirho {
    pub(super) fn seed_scalar_container_instances_chirho(&mut self) {
        // Standard instances
        for ty_name_chirho in &["Int", "Integer", "Char", "Bool", "Word"] {
            let ty_chirho = TyChirho::ConChirho(ty_name_chirho.to_string());
            self.add_seed_instance_chirho(
                haskelujah_ast_chirho::name_chirho::CompilerClassChirho::EqChirho,
                crate::class_chirho::InstDeclPartsChirho {
                    kind_tys_chirho: Vec::new(),
                    class_name_chirho: "Eq".to_string(),
                    head_ty_chirho: ty_chirho.clone(),
                    extra_head_tys_chirho: vec![],
                    context_chirho: vec![],
                },
            );
            self.add_seed_instance_chirho(
                haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ShowChirho,
                crate::class_chirho::InstDeclPartsChirho {
                    kind_tys_chirho: Vec::new(),
                    class_name_chirho: "Show".to_string(),
                    head_ty_chirho: ty_chirho.clone(),
                    extra_head_tys_chirho: vec![],
                    context_chirho: vec![],
                },
            );
        }

        for ty_name_chirho in &["Int", "Integer", "Char", "Double", "Bool", "Word"] {
            let ty_chirho = TyChirho::ConChirho(ty_name_chirho.to_string());
            self.add_seed_instance_chirho(
                haskelujah_ast_chirho::name_chirho::CompilerClassChirho::OrdChirho,
                crate::class_chirho::InstDeclPartsChirho {
                    kind_tys_chirho: Vec::new(),
                    class_name_chirho: "Ord".to_string(),
                    head_ty_chirho: ty_chirho,
                    extra_head_tys_chirho: vec![],
                    context_chirho: vec![],
                },
            );
        }

        // Ord [Char] (String ordering)
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::OrdChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Ord".to_string(),
                head_ty_chirho: TyChirho::ListChirho(Box::new(TyChirho::ConChirho(
                    "Char".to_string(),
                ))),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );

        // Eq [Char] (String equality)
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::EqChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Eq".to_string(),
                head_ty_chirho: TyChirho::ListChirho(Box::new(TyChirho::ConChirho(
                    "Char".to_string(),
                ))),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );

        // Show [Char]
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ShowChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Show".to_string(),
                head_ty_chirho: TyChirho::ListChirho(Box::new(TyChirho::ConChirho(
                    "Char".to_string(),
                ))),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );

        // Enum instances
        for ty_name_chirho in &["Int", "Integer", "Char", "Bool", "Word", "Ordering"] {
            self.add_seed_instance_chirho(
                haskelujah_ast_chirho::name_chirho::CompilerClassChirho::EnumChirho,
                crate::class_chirho::InstDeclPartsChirho {
                    kind_tys_chirho: Vec::new(),
                    class_name_chirho: "Enum".to_string(),
                    head_ty_chirho: TyChirho::ConChirho(ty_name_chirho.to_string()),
                    extra_head_tys_chirho: vec![],
                    context_chirho: vec![],
                },
            );
        }

        // Bounded instances
        for ty_name_chirho in &["Int", "Char", "Bool", "Word", "Ordering"] {
            self.add_seed_instance_chirho(
                haskelujah_ast_chirho::name_chirho::CompilerClassChirho::BoundedChirho,
                crate::class_chirho::InstDeclPartsChirho {
                    kind_tys_chirho: Vec::new(),
                    class_name_chirho: "Bounded".to_string(),
                    head_ty_chirho: TyChirho::ConChirho(ty_name_chirho.to_string()),
                    extra_head_tys_chirho: vec![],
                    context_chirho: vec![],
                },
            );
        }

        // Integral instances
        for ty_chirho in [
            TyChirho::int_chirho(),
            TyChirho::ConChirho("Integer".to_string()),
            TyChirho::ConChirho("Word".to_string()),
        ] {
            self.add_seed_instance_chirho(
                haskelujah_ast_chirho::name_chirho::CompilerClassChirho::IntegralChirho,
                crate::class_chirho::InstDeclPartsChirho {
                    kind_tys_chirho: Vec::new(),
                    class_name_chirho: "Integral".to_string(),
                    head_ty_chirho: ty_chirho,
                    extra_head_tys_chirho: vec![],
                    context_chirho: vec![],
                },
            );
        }

        // Read instances
        for ty_name_chirho in &["Int", "Integer", "Double", "Bool", "Rational"] {
            self.add_seed_instance_chirho(
                haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ReadChirho,
                crate::class_chirho::InstDeclPartsChirho {
                    kind_tys_chirho: Vec::new(),
                    class_name_chirho: "Read".to_string(),
                    head_ty_chirho: TyChirho::ConChirho(ty_name_chirho.to_string()),
                    extra_head_tys_chirho: vec![],
                    context_chirho: vec![],
                },
            );
        }

        // NFData instances for primitive/scalar types. GHC's `deepseq` package
        // provides NFData for all of these; our runtime `deepseq`/`rnf` are
        // seq-based (type-agnostic), so these are pure type-checker facts.
        // (Integer was previously missing, so `NFData [Integer]` — the type of a
        // literal list like `[1,2,3]` under defaulting — had no instance.)
        for ty_name_chirho in &[
            "Int", "Integer", "Char", "Bool", "Double", "Float", "Word", "Ordering", "Natural",
            "Int8", "Int16", "Int32", "Int64", "Word8", "Word16", "Word32", "Word64",
        ] {
            self.add_seed_instance_chirho(
                haskelujah_ast_chirho::name_chirho::CompilerClassChirho::NfDataChirho,
                crate::class_chirho::InstDeclPartsChirho {
                    kind_tys_chirho: Vec::new(),
                    class_name_chirho: "NFData".to_string(),
                    head_ty_chirho: TyChirho::ConChirho(ty_name_chirho.to_string()),
                    extra_head_tys_chirho: vec![],
                    context_chirho: vec![],
                },
            );
        }

        // instance Num Int / Integer / Word / FFI types
        for ty_chirho in [
            TyChirho::int_chirho(),
            TyChirho::ConChirho("Integer".to_string()),
            TyChirho::ConChirho("Rational".to_string()),
            TyChirho::ConChirho("Word".to_string()),
            TyChirho::ConChirho("Word8".to_string()),
            TyChirho::ConChirho("Word16".to_string()),
            TyChirho::ConChirho("Word32".to_string()),
            TyChirho::ConChirho("Word64".to_string()),
            TyChirho::ConChirho("Int8".to_string()),
            TyChirho::ConChirho("Int16".to_string()),
            TyChirho::ConChirho("Int32".to_string()),
            TyChirho::ConChirho("Int64".to_string()),
            TyChirho::ConChirho("Natural".to_string()),
            TyChirho::ConChirho("CSize".to_string()),
            TyChirho::ConChirho("CInt".to_string()),
            TyChirho::ConChirho("CChar".to_string()),
            TyChirho::ConChirho("CLong".to_string()),
            TyChirho::ConChirho("CUInt".to_string()),
            TyChirho::ConChirho("CULong".to_string()),
            TyChirho::ConChirho("Float".to_string()),
        ] {
            self.add_seed_instance_chirho(
                haskelujah_ast_chirho::name_chirho::CompilerClassChirho::NumChirho,
                crate::class_chirho::InstDeclPartsChirho {
                    kind_tys_chirho: Vec::new(),
                    class_name_chirho: "Num".to_string(),
                    head_ty_chirho: ty_chirho,
                    extra_head_tys_chirho: vec![],
                    context_chirho: vec![],
                },
            );
        }

        // instance Num Double
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::NumChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Num".to_string(),
                head_ty_chirho: TyChirho::ConChirho("Double".to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );

        // instance Fractional Double
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::FractionalChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Fractional".to_string(),
                head_ty_chirho: TyChirho::ConChirho("Double".to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::FractionalChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Fractional".to_string(),
                head_ty_chirho: TyChirho::ConChirho("Rational".to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );

        // instance Eq Double
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::EqChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Eq".to_string(),
                head_ty_chirho: TyChirho::ConChirho("Double".to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::EqChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Eq".to_string(),
                head_ty_chirho: TyChirho::ConChirho("Rational".to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );

        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::OrdChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Ord".to_string(),
                head_ty_chirho: TyChirho::ConChirho("Rational".to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );

        // instance Show Double
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ShowChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Show".to_string(),
                head_ty_chirho: TyChirho::ConChirho("Double".to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ShowChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Show".to_string(),
                head_ty_chirho: TyChirho::ConChirho("Rational".to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::RealChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Real".to_string(),
                head_ty_chirho: TyChirho::ConChirho("Rational".to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::RealFracChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "RealFrac".to_string(),
                head_ty_chirho: TyChirho::ConChirho("Rational".to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );

        // instance Eq Ordering
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::EqChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Eq".to_string(),
                head_ty_chirho: TyChirho::ConChirho("Ordering".to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );

        // instance Show Ordering
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ShowChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Show".to_string(),
                head_ty_chirho: TyChirho::ConChirho("Ordering".to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );

        // instance Ord Ordering
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::OrdChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Ord".to_string(),
                head_ty_chirho: TyChirho::ConChirho("Ordering".to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );

        // Bulk instances for Word/Int/C FFI integer types: Eq, Ord, Show,
        // Bounded, Enum, Real, Integral for all fixed-width numeric and C types.
        // Float is handled separately below; it is Real/Fractional, but not Integral.
        let ffi_numeric_types_chirho: &[&str] = &[
            "Word8", "Word16", "Word32", "Word64", "Int8", "Int16", "Int32", "Int64", "Natural",
            "CSize", "CInt", "CChar", "CLong", "CUInt", "CULong",
        ];
        let ffi_classes_chirho: &[(
            &str,
            Option<haskelujah_ast_chirho::name_chirho::CompilerClassChirho>,
        )] = &[
            (
                "Eq",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::EqChirho),
            ),
            (
                "Ord",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::OrdChirho),
            ),
            (
                "Show",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ShowChirho),
            ),
            (
                "Bounded",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::BoundedChirho),
            ),
            (
                "Enum",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::EnumChirho),
            ),
            (
                "Real",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::RealChirho),
            ),
            (
                "Integral",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::IntegralChirho),
            ),
            (
                "Read",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ReadChirho),
            ),
            (
                "Bits",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::BitsChirho),
            ),
            (
                "FiniteBits",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::FiniteBitsChirho),
            ),
            ("Storable", None),
        ];
        for &ty_name_chirho in ffi_numeric_types_chirho {
            for &(class_name_chirho, class_identity_chirho) in ffi_classes_chirho {
                self.add_seed_instance_reference_chirho(
                    class_identity_chirho,
                    crate::class_chirho::InstDeclPartsChirho {
                        kind_tys_chirho: Vec::new(),
                        class_name_chirho: class_name_chirho.to_string(),
                        head_ty_chirho: TyChirho::ConChirho(ty_name_chirho.to_string()),
                        extra_head_tys_chirho: vec![],
                        context_chirho: vec![],
                    },
                );
            }
        }
        for (class_chirho, class_identity_chirho) in [
            (
                "Eq",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::EqChirho),
            ),
            (
                "Ord",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::OrdChirho),
            ),
            (
                "Show",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ShowChirho),
            ),
            (
                "Enum",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::EnumChirho),
            ),
            (
                "Read",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ReadChirho),
            ),
            (
                "Real",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::RealChirho),
            ),
            ("Storable", None),
        ] {
            self.add_seed_instance_reference_chirho(
                class_identity_chirho,
                crate::class_chirho::InstDeclPartsChirho {
                    kind_tys_chirho: Vec::new(),
                    class_name_chirho: class_chirho.to_string(),
                    head_ty_chirho: TyChirho::ConChirho("Float".to_string()),
                    extra_head_tys_chirho: vec![],
                    context_chirho: vec![],
                },
            );
        }
        // Fractional/Floating/RealFloat for Float
        for (class_chirho, class_identity_chirho) in [
            (
                "Fractional",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::FractionalChirho),
            ),
            (
                "Floating",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::FloatingChirho),
            ),
            (
                "RealFrac",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::RealFracChirho),
            ),
            (
                "RealFloat",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::RealFloatChirho),
            ),
        ] {
            self.add_seed_instance_reference_chirho(
                class_identity_chirho,
                crate::class_chirho::InstDeclPartsChirho {
                    kind_tys_chirho: Vec::new(),
                    class_name_chirho: class_chirho.to_string(),
                    head_ty_chirho: TyChirho::ConChirho("Float".to_string()),
                    extra_head_tys_chirho: vec![],
                    context_chirho: vec![],
                },
            );
        }

        // Ground instance: Eq String (i.e. Eq [Char])
        // This is a special case of the conditional Eq [a] instance
        // for the common String = [Char] type, avoiding the need for
        // full conditional instance dictionary support.
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::EqChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Eq".to_string(),
                head_ty_chirho: TyChirho::ConChirho("[Char]".to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ShowChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Show".to_string(),
                head_ty_chirho: TyChirho::ConChirho("[Char]".to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );

        // Ground instance: Show [Int]
        // Avoids needing full conditional instance dictionary resolution
        // for the common case of showing integer lists.
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ShowChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Show".to_string(),
                head_ty_chirho: TyChirho::ConChirho("[Int]".to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );

        // instance Eq a => Eq [a]
        let a_var_chirho = TyVarChirho(9900);
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::EqChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Eq".to_string(),
                head_ty_chirho: TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_var_chirho))),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![PredChirho::new_chirho(
                    "Eq",
                    TyChirho::VarChirho(a_var_chirho),
                )],
            },
        );

        // instance Show a => Show [a]
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ShowChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Show".to_string(),
                head_ty_chirho: TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_var_chirho))),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![PredChirho::new_chirho(
                    "Show",
                    TyChirho::VarChirho(a_var_chirho),
                )],
            },
        );

        // instance NFData a => NFData [a]
        // (GHC provides this; needed so `NFData [Integer]` from `deepseq [1,2,3] y`
        // resolves via the element instance. deepseq is seq-based at runtime.)
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::NfDataChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "NFData".to_string(),
                head_ty_chirho: TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_var_chirho))),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![PredChirho::new_chirho(
                    "NFData",
                    TyChirho::VarChirho(a_var_chirho),
                )],
            },
        );

        let b_var_chirho = TyVarChirho(9901);

        // instance Show a => Show (Maybe a)
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ShowChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Show".to_string(),
                head_ty_chirho: TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Maybe".to_string())),
                    Box::new(TyChirho::VarChirho(a_var_chirho)),
                ),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![PredChirho::new_chirho(
                    "Show",
                    TyChirho::VarChirho(a_var_chirho),
                )],
            },
        );

        // instance (C a, C b) => C (Either a b) for the derived classes base
        // gives Either. `Maybe` had these; `Either` did not, so `print (Right 1
        // :: Either String Int)` looked like an unsolved constraint.
        for (class_chirho, class_identity_chirho) in [
            (
                "Show",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ShowChirho),
            ),
            (
                "Eq",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::EqChirho),
            ),
            (
                "Ord",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::OrdChirho),
            ),
            (
                "Read",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ReadChirho),
            ),
        ] {
            self.add_seed_instance_reference_chirho(
                class_identity_chirho,
                crate::class_chirho::InstDeclPartsChirho {
                    kind_tys_chirho: Vec::new(),
                    class_name_chirho: class_chirho.to_string(),
                    head_ty_chirho: TyChirho::AppChirho(
                        Box::new(TyChirho::AppChirho(
                            Box::new(TyChirho::ConChirho("Either".to_string())),
                            Box::new(TyChirho::VarChirho(a_var_chirho)),
                        )),
                        Box::new(TyChirho::VarChirho(b_var_chirho)),
                    ),
                    extra_head_tys_chirho: vec![],
                    context_chirho: vec![
                        PredChirho::new_chirho(class_chirho, TyChirho::VarChirho(a_var_chirho)),
                        PredChirho::new_chirho(class_chirho, TyChirho::VarChirho(b_var_chirho)),
                    ],
                },
            );
        }

        // instance Ord a => Ord [a]
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::OrdChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Ord".to_string(),
                head_ty_chirho: TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_var_chirho))),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![PredChirho::new_chirho(
                    "Ord",
                    TyChirho::VarChirho(a_var_chirho),
                )],
            },
        );

        // instance Eq a => Eq (Maybe a)
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::EqChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Eq".to_string(),
                head_ty_chirho: TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Maybe".to_string())),
                    Box::new(TyChirho::VarChirho(a_var_chirho)),
                ),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![PredChirho::new_chirho(
                    "Eq",
                    TyChirho::VarChirho(a_var_chirho),
                )],
            },
        );

        // instance Ord a => Ord (Maybe a)
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::OrdChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Ord".to_string(),
                head_ty_chirho: TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Maybe".to_string())),
                    Box::new(TyChirho::VarChirho(a_var_chirho)),
                ),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![PredChirho::new_chirho(
                    "Ord",
                    TyChirho::VarChirho(a_var_chirho),
                )],
            },
        );

        // instance Read a => Read [a]
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ReadChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Read".to_string(),
                head_ty_chirho: TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_var_chirho))),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![PredChirho::new_chirho(
                    "Read",
                    TyChirho::VarChirho(a_var_chirho),
                )],
            },
        );

        // instance Read a => Read (Maybe a)
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ReadChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Read".to_string(),
                head_ty_chirho: TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Maybe".to_string())),
                    Box::new(TyChirho::VarChirho(a_var_chirho)),
                ),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![PredChirho::new_chirho(
                    "Read",
                    TyChirho::VarChirho(a_var_chirho),
                )],
            },
        );

        // Tuple instances, pairs through 15-tuples (row 604).
        self.seed_tuple_instances_chirho();

        // instance Num a => Num [a] -- not standard, but avoids false errors
        // instance Enum a => Enum [a] -- not standard either, skip

        // instance Bounded a => Bounded (Maybe a) -- not standard, skip
    }
}

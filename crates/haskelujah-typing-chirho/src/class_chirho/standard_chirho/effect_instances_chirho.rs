// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Contiguous standard-seed segment; order is owned by the parent dispatcher.

use crate::ty_chirho::{TyChirho, TyVarChirho};

use super::super::{ClassEnvChirho, PredChirho};

impl ClassEnvChirho {
    pub(super) fn seed_effect_instances_chirho(&mut self) {
        // ── Functor / Applicative / Monad instances ──

        // instance Functor Maybe
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::FunctorChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Functor".to_string(),
                head_ty_chirho: TyChirho::ConChirho("Maybe".to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );

        // instance Functor []
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::FunctorChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Functor".to_string(),
                head_ty_chirho: TyChirho::ConChirho("[]".to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );

        // instance Applicative Maybe
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ApplicativeChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Applicative".to_string(),
                head_ty_chirho: TyChirho::ConChirho("Maybe".to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );

        // instance Monad Maybe
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::MonadChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Monad".to_string(),
                head_ty_chirho: TyChirho::ConChirho("Maybe".to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );

        // instance Functor IO
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::FunctorChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Functor".to_string(),
                head_ty_chirho: TyChirho::ConChirho("IO".to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );

        // instance Applicative IO
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ApplicativeChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Applicative".to_string(),
                head_ty_chirho: TyChirho::ConChirho("IO".to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );

        // instance Monad IO
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::MonadChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Monad".to_string(),
                head_ty_chirho: TyChirho::ConChirho("IO".to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );

        // instance Applicative []
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ApplicativeChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Applicative".to_string(),
                head_ty_chirho: TyChirho::ConChirho("[]".to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );

        // instance Monad []
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::MonadChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Monad".to_string(),
                head_ty_chirho: TyChirho::ConChirho("[]".to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );

        // instance Functor/Applicative/Monad Identity
        for (class_chirho, class_identity_chirho) in [
            (
                "Functor",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::FunctorChirho),
            ),
            (
                "Applicative",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ApplicativeChirho),
            ),
            (
                "Monad",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::MonadChirho),
            ),
        ] {
            self.add_seed_instance_reference_chirho(
                class_identity_chirho,
                crate::class_chirho::InstDeclPartsChirho {
                    kind_tys_chirho: Vec::new(),
                    class_name_chirho: class_chirho.to_string(),
                    head_ty_chirho: TyChirho::ConChirho("Identity".to_string()),
                    extra_head_tys_chirho: vec![],
                    context_chirho: vec![],
                },
            );
        }

        let state_token_var_chirho = TyVarChirho(4000);
        let state_monad_var_chirho = TyVarChirho(4001);
        let state_t_head_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("StateT".to_string())),
                Box::new(TyChirho::VarChirho(state_token_var_chirho)),
            )),
            Box::new(TyChirho::VarChirho(state_monad_var_chirho)),
        );

        // instance Functor m => Functor (StateT s m)
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::FunctorChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Functor".to_string(),
                head_ty_chirho: state_t_head_ty_chirho.clone(),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![PredChirho::compiler_chirho(
                    haskelujah_ast_chirho::name_chirho::CompilerClassChirho::FunctorChirho,
                    TyChirho::VarChirho(state_monad_var_chirho),
                )],
            },
        );

        // instance Applicative m => Applicative (StateT s m)
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ApplicativeChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Applicative".to_string(),
                head_ty_chirho: state_t_head_ty_chirho.clone(),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![PredChirho::compiler_chirho(
                    haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ApplicativeChirho,
                    TyChirho::VarChirho(state_monad_var_chirho),
                )],
            },
        );

        // instance Monad m => Monad (StateT s m)
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::MonadChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Monad".to_string(),
                head_ty_chirho: state_t_head_ty_chirho,
                extra_head_tys_chirho: vec![],
                context_chirho: vec![PredChirho::compiler_chirho(
                    haskelujah_ast_chirho::name_chirho::CompilerClassChirho::MonadChirho,
                    TyChirho::VarChirho(state_monad_var_chirho),
                )],
            },
        );

        let reader_env_var_chirho = TyVarChirho(4003);
        let reader_monad_var_chirho = TyVarChirho(4004);
        let reader_t_head_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ReaderT".to_string())),
                Box::new(TyChirho::VarChirho(reader_env_var_chirho)),
            )),
            Box::new(TyChirho::VarChirho(reader_monad_var_chirho)),
        );

        // instance Functor m => Functor (ReaderT r m)
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::FunctorChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Functor".to_string(),
                head_ty_chirho: reader_t_head_ty_chirho.clone(),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![PredChirho::compiler_chirho(
                    haskelujah_ast_chirho::name_chirho::CompilerClassChirho::FunctorChirho,
                    TyChirho::VarChirho(reader_monad_var_chirho),
                )],
            },
        );

        // instance Applicative m => Applicative (ReaderT r m)
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ApplicativeChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Applicative".to_string(),
                head_ty_chirho: reader_t_head_ty_chirho.clone(),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![PredChirho::compiler_chirho(
                    haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ApplicativeChirho,
                    TyChirho::VarChirho(reader_monad_var_chirho),
                )],
            },
        );

        // instance Monad m => Monad (ReaderT r m)
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::MonadChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Monad".to_string(),
                head_ty_chirho: reader_t_head_ty_chirho.clone(),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![PredChirho::compiler_chirho(
                    haskelujah_ast_chirho::name_chirho::CompilerClassChirho::MonadChirho,
                    TyChirho::VarChirho(reader_monad_var_chirho),
                )],
            },
        );

        // instance MonadTrans (ReaderT r)
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::MonadTransChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "MonadTrans".to_string(),
                head_ty_chirho: TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("ReaderT".to_string())),
                    Box::new(TyChirho::VarChirho(reader_env_var_chirho)),
                ),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );

        let maybe_t_monad_var_chirho = TyVarChirho(4005);
        let maybe_t_head_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("MaybeT".to_string())),
            Box::new(TyChirho::VarChirho(maybe_t_monad_var_chirho)),
        );

        // instance Functor m => Functor (MaybeT m)
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::FunctorChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Functor".to_string(),
                head_ty_chirho: maybe_t_head_ty_chirho.clone(),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![PredChirho::compiler_chirho(
                    haskelujah_ast_chirho::name_chirho::CompilerClassChirho::FunctorChirho,
                    TyChirho::VarChirho(maybe_t_monad_var_chirho),
                )],
            },
        );

        // instance Applicative m => Applicative (MaybeT m)
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ApplicativeChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Applicative".to_string(),
                head_ty_chirho: maybe_t_head_ty_chirho.clone(),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![PredChirho::compiler_chirho(
                    haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ApplicativeChirho,
                    TyChirho::VarChirho(maybe_t_monad_var_chirho),
                )],
            },
        );

        // instance Monad m => Monad (MaybeT m)
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::MonadChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Monad".to_string(),
                head_ty_chirho: maybe_t_head_ty_chirho.clone(),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![PredChirho::compiler_chirho(
                    haskelujah_ast_chirho::name_chirho::CompilerClassChirho::MonadChirho,
                    TyChirho::VarChirho(maybe_t_monad_var_chirho),
                )],
            },
        );

        // instance MonadTrans MaybeT
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::MonadTransChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "MonadTrans".to_string(),
                head_ty_chirho: TyChirho::ConChirho("MaybeT".to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );

        let st_token_var_chirho = TyVarChirho(4002);
        let st_head_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("ST".to_string())),
            Box::new(TyChirho::VarChirho(st_token_var_chirho)),
        );

        // instance Functor (ST s)
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::FunctorChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Functor".to_string(),
                head_ty_chirho: st_head_ty_chirho.clone(),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );

        // instance Applicative (ST s)
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ApplicativeChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Applicative".to_string(),
                head_ty_chirho: st_head_ty_chirho.clone(),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );

        // instance Monad (ST s)
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::MonadChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Monad".to_string(),
                head_ty_chirho: st_head_ty_chirho,
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );

        // instance Bits Int
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::BitsChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Bits".to_string(),
                head_ty_chirho: TyChirho::ConChirho("Int".to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );

        // instance Bits Word
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::BitsChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Bits".to_string(),
                head_ty_chirho: TyChirho::ConChirho("Word".to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );

        // instance FiniteBits Int
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::FiniteBitsChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "FiniteBits".to_string(),
                head_ty_chirho: TyChirho::ConChirho("Int".to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );

        // instance FiniteBits Word
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::FiniteBitsChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "FiniteBits".to_string(),
                head_ty_chirho: TyChirho::ConChirho("Word".to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );
    }
}

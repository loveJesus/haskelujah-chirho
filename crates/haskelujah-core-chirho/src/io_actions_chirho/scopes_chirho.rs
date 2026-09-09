// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Legacy exception/transaction primitives own the execution scope. Give them
//! computations returning result packets, not dormant action constructors.
//! Payloads remain lazy, including successful results passing through handlers.
//! Workflow: testing-chirho/execution-oracles-chirho.md.

use super::{
    ActionsChirho, AltConChirho, CoreAltChirho, CoreExprChirho, RESULT_CON_CHIRHO, app_chirho,
    lambda_chirho, result_chirho, var_chirho,
};

impl ActionsChirho {
    pub(super) fn scoped_chirho(
        &mut self,
        name_chirho: &'static str,
        arguments_chirho: Vec<CoreExprChirho>,
    ) -> CoreExprChirho {
        let mut arguments_chirho = arguments_chirho.into_iter();
        let first_chirho = arguments_chirho.next().unwrap();
        if name_chirho == "mfixIO#" {
            return self.fix_result_chirho(first_chirho);
        }
        let first_chirho = self.run_chirho(first_chirho);
        match name_chirho {
            "catch#" => {
                let handler_chirho = arguments_chirho.next().unwrap();
                let exception_chirho = self.binder_chirho("io_exception");
                let handled_chirho =
                    self.run_chirho(app_chirho(handler_chirho, var_chirho(&exception_chirho)));
                CoreExprChirho::PrimOpChirho {
                    name_chirho: name_chirho.to_string(),
                    args_chirho: vec![
                        first_chirho,
                        lambda_chirho(exception_chirho, handled_chirho),
                    ],
                }
            }
            "try#" => {
                let exception_chirho = self.binder_chirho("io_exception");
                let packet_chirho = self.binder_chirho("io_tried_packet");
                let value_chirho = self.binder_chirho("io_tried_value");
                let successful_chirho = result_chirho(CoreExprChirho::ConAppChirho {
                    con_name_chirho: "Right".to_string(),
                    args_chirho: vec![var_chirho(&value_chirho)],
                });
                let successful_chirho = self.unpack_chirho(
                    var_chirho(&packet_chirho),
                    RESULT_CON_CHIRHO,
                    value_chirho,
                    successful_chirho,
                );
                let failed_chirho = result_chirho(CoreExprChirho::ConAppChirho {
                    con_name_chirho: "Left".to_string(),
                    args_chirho: vec![var_chirho(&exception_chirho)],
                });
                let outcome_chirho = self.binder_chirho("io_tried_outcome");
                self.case_chirho(
                    CoreExprChirho::PrimOpChirho {
                        name_chirho: name_chirho.to_string(),
                        args_chirho: vec![first_chirho],
                    },
                    outcome_chirho,
                    vec![
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("Left".to_string()),
                            binders_chirho: vec![exception_chirho],
                            rhs_chirho: failed_chirho,
                        },
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("Right".to_string()),
                            binders_chirho: vec![packet_chirho],
                            rhs_chirho: successful_chirho,
                        },
                    ],
                )
            }
            "bracket#" => {
                let release_chirho =
                    self.packet_continuation_chirho(arguments_chirho.next().unwrap());
                let body_chirho = self.packet_continuation_chirho(arguments_chirho.next().unwrap());
                CoreExprChirho::PrimOpChirho {
                    name_chirho: name_chirho.to_string(),
                    args_chirho: vec![first_chirho, release_chirho, body_chirho],
                }
            }
            "finally#" => {
                let cleanup_chirho = self.run_chirho(arguments_chirho.next().unwrap());
                CoreExprChirho::PrimOpChirho {
                    name_chirho: name_chirho.to_string(),
                    args_chirho: vec![first_chirho, cleanup_chirho],
                }
            }
            "atomically#" => CoreExprChirho::PrimOpChirho {
                name_chirho: name_chirho.to_string(),
                args_chirho: vec![first_chirho],
            },
            _ => unreachable!("operation table admits only supported scopes"),
        }
    }

    /// Each execution creates its own knot. The function receives the lazy
    /// result payload, not its dormant action or the result packet itself.
    fn fix_result_chirho(&mut self, function_chirho: CoreExprChirho) -> CoreExprChirho {
        let packet_chirho = self.binder_chirho("io_fixed_packet");
        let value_chirho = self.binder_chirho("io_fixed_value");
        let payload_chirho = self.binder_chirho("io_fixed_payload");
        let packet_rhs_chirho =
            self.run_chirho(app_chirho(function_chirho, var_chirho(&value_chirho)));
        let value_rhs_chirho = self.unpack_chirho(
            var_chirho(&packet_chirho),
            RESULT_CON_CHIRHO,
            payload_chirho.clone(),
            var_chirho(&payload_chirho),
        );
        CoreExprChirho::LetChirho {
            rec_chirho: true,
            binds_chirho: vec![
                (packet_chirho.clone(), packet_rhs_chirho),
                (value_chirho, value_rhs_chirho),
            ],
            body_chirho: Box::new(var_chirho(&packet_chirho)),
        }
    }

    fn packet_continuation_chirho(&mut self, function_chirho: CoreExprChirho) -> CoreExprChirho {
        let packet_chirho = self.binder_chirho("io_acquired_packet");
        let resource_chirho = self.binder_chirho("io_resource");
        let applied_chirho =
            self.run_chirho(app_chirho(function_chirho, var_chirho(&resource_chirho)));
        let unpacked_chirho = self.unpack_chirho(
            var_chirho(&packet_chirho),
            RESULT_CON_CHIRHO,
            resource_chirho,
            applied_chirho,
        );
        lambda_chirho(packet_chirho, unpacked_chirho)
    }
}

// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! Golden CST tests that verify real node kinds and token text.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use haskelujah_parser_chirho::cst_parser_chirho::ParserChirho;
use haskelujah_span_chirho::FileIdChirho;
use haskelujah_syntax_chirho::cst_chirho::SyntaxKindChirho;
use haskelujah_syntax_chirho::green_chirho::{GreenElementChirho, GreenNodeChirho};
use haskelujah_test_harness_chirho::{assert_golden_chirho, bless_golden_chirho};

const JOHN_3_16_COMMENT_CHIRHO: &str =
    "<!-- For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. -->\n\n";

#[derive(Clone, Copy)]
struct ExpectedNodeAtPathChirho {
    node_path_chirho: &'static [usize],
    expected_kind_chirho: SyntaxKindChirho,
}

struct ParseGoldenCaseChirho {
    name_chirho: &'static str,
    source_chirho: &'static str,
    expected_nodes_chirho: &'static [ExpectedNodeAtPathChirho],
}

fn golden_dir_chirho() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../test-data-chirho/golden-parse-chirho")
}

fn golden_cases_chirho() -> Vec<ParseGoldenCaseChirho> {
    vec![
        ParseGoldenCaseChirho {
            name_chirho: "type_sig_chirho",
            source_chirho: "module TypeSigChirho where\nfooChirho :: Int -> String\n",
            expected_nodes_chirho: &[
                ExpectedNodeAtPathChirho {
                    node_path_chirho: &[0],
                    expected_kind_chirho: SyntaxKindChirho::ModuleHeaderChirho,
                },
                ExpectedNodeAtPathChirho {
                    node_path_chirho: &[1],
                    expected_kind_chirho: SyntaxKindChirho::TypeSigDeclChirho,
                },
            ],
        },
        ParseGoldenCaseChirho {
            name_chirho: "data_decl_chirho",
            source_chirho:
                "module DataDeclChirho where\ndata ColorChirho = RedChirho | GreenChirho | BlueChirho\n",
            expected_nodes_chirho: &[
                ExpectedNodeAtPathChirho {
                    node_path_chirho: &[0],
                    expected_kind_chirho: SyntaxKindChirho::ModuleHeaderChirho,
                },
                ExpectedNodeAtPathChirho {
                    node_path_chirho: &[1],
                    expected_kind_chirho: SyntaxKindChirho::DataDeclChirho,
                },
            ],
        },
        ParseGoldenCaseChirho {
            name_chirho: "fun_bind_chirho",
            source_chirho:
                "module FunBindChirho where\nfChirho xChirho yChirho = xChirho + yChirho\n",
            expected_nodes_chirho: &[
                ExpectedNodeAtPathChirho {
                    node_path_chirho: &[0],
                    expected_kind_chirho: SyntaxKindChirho::ModuleHeaderChirho,
                },
                ExpectedNodeAtPathChirho {
                    node_path_chirho: &[1],
                    expected_kind_chirho: SyntaxKindChirho::FunBindChirho,
                },
            ],
        },
        ParseGoldenCaseChirho {
            name_chirho: "lambda_chirho",
            source_chirho:
                "module LambdaChirho where\nlambdaValueChirho = \\xChirho -> xChirho\n",
            expected_nodes_chirho: &[
                ExpectedNodeAtPathChirho {
                    node_path_chirho: &[0],
                    expected_kind_chirho: SyntaxKindChirho::ModuleHeaderChirho,
                },
                ExpectedNodeAtPathChirho {
                    node_path_chirho: &[1],
                    expected_kind_chirho: SyntaxKindChirho::FunBindChirho,
                },
            ],
        },
        ParseGoldenCaseChirho {
            name_chirho: "case_chirho",
            source_chirho: "module CaseChirho where\ncaseValueChirho xChirho = case xChirho of { True -> 1; False -> 0 }\n",
            expected_nodes_chirho: &[
                ExpectedNodeAtPathChirho {
                    node_path_chirho: &[0],
                    expected_kind_chirho: SyntaxKindChirho::ModuleHeaderChirho,
                },
                ExpectedNodeAtPathChirho {
                    node_path_chirho: &[1],
                    expected_kind_chirho: SyntaxKindChirho::FunBindChirho,
                },
            ],
        },
        ParseGoldenCaseChirho {
            name_chirho: "do_block_chirho",
            source_chirho: "module DoBlockChirho where\nmainChirho = do { valueChirho <- pure 1; pure valueChirho }\n",
            expected_nodes_chirho: &[
                ExpectedNodeAtPathChirho {
                    node_path_chirho: &[0],
                    expected_kind_chirho: SyntaxKindChirho::ModuleHeaderChirho,
                },
                ExpectedNodeAtPathChirho {
                    node_path_chirho: &[1],
                    expected_kind_chirho: SyntaxKindChirho::FunBindChirho,
                },
            ],
        },
        ParseGoldenCaseChirho {
            name_chirho: "let_expr_chirho",
            source_chirho:
                "module LetExprChirho where\nletValueChirho = let innerValueChirho = 42 in innerValueChirho\n",
            expected_nodes_chirho: &[
                ExpectedNodeAtPathChirho {
                    node_path_chirho: &[0],
                    expected_kind_chirho: SyntaxKindChirho::ModuleHeaderChirho,
                },
                ExpectedNodeAtPathChirho {
                    node_path_chirho: &[1],
                    expected_kind_chirho: SyntaxKindChirho::FunBindChirho,
                },
            ],
        },
        ParseGoldenCaseChirho {
            name_chirho: "list_comp_chirho",
            source_chirho: "module ListCompChirho where\nlistCompValueChirho = [xChirho + 1 | xChirho <- [1, 2, 3]]\n",
            expected_nodes_chirho: &[
                ExpectedNodeAtPathChirho {
                    node_path_chirho: &[0],
                    expected_kind_chirho: SyntaxKindChirho::ModuleHeaderChirho,
                },
                ExpectedNodeAtPathChirho {
                    node_path_chirho: &[1],
                    expected_kind_chirho: SyntaxKindChirho::FunBindChirho,
                },
            ],
        },
        ParseGoldenCaseChirho {
            name_chirho: "record_chirho",
            source_chirho: "module RecordChirho where\ndata PersonChirho = PersonChirho { nameChirho :: String, ageChirho :: Int }\nrecordValueChirho = PersonChirho { nameChirho = \"Ada\", ageChirho = 36 }\n",
            expected_nodes_chirho: &[
                ExpectedNodeAtPathChirho {
                    node_path_chirho: &[0],
                    expected_kind_chirho: SyntaxKindChirho::ModuleHeaderChirho,
                },
                ExpectedNodeAtPathChirho {
                    node_path_chirho: &[1],
                    expected_kind_chirho: SyntaxKindChirho::DataDeclChirho,
                },
                ExpectedNodeAtPathChirho {
                    node_path_chirho: &[2],
                    expected_kind_chirho: SyntaxKindChirho::FunBindChirho,
                },
            ],
        },
        ParseGoldenCaseChirho {
            name_chirho: "class_decl_chirho",
            source_chirho: "module ClassDeclChirho where\nclass DescribableChirho aChirho where { describeChirho :: aChirho -> String }\n",
            expected_nodes_chirho: &[
                ExpectedNodeAtPathChirho {
                    node_path_chirho: &[0],
                    expected_kind_chirho: SyntaxKindChirho::ModuleHeaderChirho,
                },
                ExpectedNodeAtPathChirho {
                    node_path_chirho: &[1],
                    expected_kind_chirho: SyntaxKindChirho::ClassDeclChirho,
                },
            ],
        },
    ]
}

fn parse_to_green_chirho(source_chirho: &str) -> Arc<GreenNodeChirho> {
    ParserChirho::new_chirho(source_chirho, FileIdChirho::SYNTHETIC_CHIRHO).parse_chirho()
}

fn push_indent_chirho(output_chirho: &mut String, depth_chirho: usize) {
    for _indent_level_chirho in 0..depth_chirho {
        output_chirho.push_str("  ");
    }
}

fn escaped_token_text_chirho(text_chirho: &str) -> String {
    text_chirho
        .replace('\\', "\\\\")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

fn render_green_node_chirho(
    node_chirho: &GreenNodeChirho,
    depth_chirho: usize,
    output_chirho: &mut String,
) {
    push_indent_chirho(output_chirho, depth_chirho);
    output_chirho.push_str(&format!("{:?}\n", node_chirho.kind_chirho()));

    for child_chirho in node_chirho.children_chirho() {
        match child_chirho {
            GreenElementChirho::NodeChirho(child_node_chirho) => {
                render_green_node_chirho(child_node_chirho, depth_chirho + 1, output_chirho);
            }
            GreenElementChirho::TokenChirho(token_chirho) => {
                push_indent_chirho(output_chirho, depth_chirho + 1);
                output_chirho.push_str(&format!(
                    "{:?} \"{}\"\n",
                    token_chirho.kind_chirho(),
                    escaped_token_text_chirho(token_chirho.text_chirho()),
                ));
            }
        }
    }
}

fn render_green_tree_chirho(root_chirho: &GreenNodeChirho) -> String {
    let mut output_chirho = String::from(JOHN_3_16_COMMENT_CHIRHO);
    render_green_node_chirho(root_chirho, 0, &mut output_chirho);
    output_chirho
}

fn node_children_chirho(node_chirho: &GreenNodeChirho) -> Vec<&GreenNodeChirho> {
    node_chirho
        .children_chirho()
        .iter()
        .filter_map(|child_chirho| match child_chirho {
            GreenElementChirho::NodeChirho(node_chirho) => Some(node_chirho.as_ref()),
            GreenElementChirho::TokenChirho(_) => None,
        })
        .collect()
}

fn node_at_path_chirho<'node_chirho>(
    root_chirho: &'node_chirho GreenNodeChirho,
    node_path_chirho: &[usize],
) -> &'node_chirho GreenNodeChirho {
    let mut current_node_chirho = root_chirho;
    for &child_index_chirho in node_path_chirho {
        let child_nodes_chirho = node_children_chirho(current_node_chirho);
        current_node_chirho = child_nodes_chirho
            .get(child_index_chirho)
            .unwrap_or_else(|| {
                panic!(
                    "missing node path {:?} at segment {} under {:?}",
                    node_path_chirho,
                    child_index_chirho,
                    current_node_chirho.kind_chirho()
                )
            });
    }
    current_node_chirho
}

fn assert_expected_nodes_chirho(
    root_chirho: &GreenNodeChirho,
    case_chirho: &ParseGoldenCaseChirho,
) {
    for expected_node_chirho in case_chirho.expected_nodes_chirho {
        let actual_node_chirho =
            node_at_path_chirho(root_chirho, expected_node_chirho.node_path_chirho);
        assert_eq!(
            actual_node_chirho.kind_chirho(),
            expected_node_chirho.expected_kind_chirho,
            "case `{}` had unexpected node kind at path {:?}",
            case_chirho.name_chirho,
            expected_node_chirho.node_path_chirho,
        );
    }
}

fn expected_path_chirho(golden_dir_chirho: &Path, case_chirho: &ParseGoldenCaseChirho) -> PathBuf {
    golden_dir_chirho.join(format!("{}.expected", case_chirho.name_chirho))
}

#[test]
fn golden_parse_all_chirho() {
    let bless_chirho = std::env::var("BLESS_CHIRHO").is_ok();
    let golden_dir_chirho = golden_dir_chirho();
    std::fs::create_dir_all(&golden_dir_chirho).expect("should create golden parse directory");

    let cases_chirho = golden_cases_chirho();
    let mut failures_chirho: Vec<String> = Vec::new();

    for case_chirho in &cases_chirho {
        let root_chirho = parse_to_green_chirho(case_chirho.source_chirho);
        assert_expected_nodes_chirho(root_chirho.as_ref(), case_chirho);
        let actual_chirho = render_green_tree_chirho(root_chirho.as_ref());
        let expected_path_chirho = expected_path_chirho(&golden_dir_chirho, case_chirho);

        if bless_chirho {
            bless_golden_chirho(&actual_chirho, &expected_path_chirho)
                .expect("should bless parse golden");
        } else if let Err(mismatch_chirho) =
            assert_golden_chirho(&actual_chirho, &expected_path_chirho)
        {
            failures_chirho.push(format!(
                "--- {} ---\n{}",
                case_chirho.name_chirho, mismatch_chirho
            ));
        }
    }

    if !failures_chirho.is_empty() {
        panic!(
            "{} golden parse test(s) failed:\n\n{}",
            failures_chirho.len(),
            failures_chirho.join("\n\n")
        );
    }
}

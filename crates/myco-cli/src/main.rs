use std::{env, fs, process::ExitCode};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let Some(command) = args.next() else {
        return Err(usage());
    };

    match command.as_str() {
        "check" => {
            let path = args.next().ok_or_else(usage)?;
            if args.next().is_some() {
                return Err(usage());
            }

            let source =
                fs::read_to_string(&path).map_err(|err| format!("failed to read {path}: {err}"))?;

            match myco_core::pipeline::load_model(&source) {
                Ok(model) => {
                    let summary = model.summary();
                    println!(
                        "ok: model '{}' parsed, validated, and lowered ({} quantities, {} equations, {} slots)",
                        summary.name, summary.quantity_count, summary.relation_count, summary.slot_count
                    );
                    Ok(())
                }
                Err(diagnostics) => fail_with_diagnostics(&path, diagnostics),
            }
        }
        "inspect" => {
            let path = args.next().ok_or_else(usage)?;
            if args.next().is_some() {
                return Err(usage());
            }

            let source =
                fs::read_to_string(&path).map_err(|err| format!("failed to read {path}: {err}"))?;

            match myco_core::pipeline::load_model(&source) {
                Ok(model) => {
                    let summary = model.summary();
                    println!("model: {}", summary.name);
                    println!(
                        "counts: quantities={}, relations={}, slots={}, external={}, state={}, node={}, temporal={}",
                        summary.quantity_count,
                        summary.relation_count,
                        summary.slot_count,
                        summary.external_count,
                        summary.state_count,
                        summary.node_count,
                        summary.temporal_count
                    );
                    println!("quantities: {}", summary.quantity_names.join(", "));
                    println!("relations: {}", summary.relation_names.join(", "));
                    println!("slots: {}", summary.slot_names.join(", "));
                    Ok(())
                }
                Err(diagnostics) => fail_with_diagnostics(&path, diagnostics),
            }
        }
        _ => Err(usage()),
    }
}

fn fail_with_diagnostics(
    path: &str,
    diagnostics: Vec<myco_core::diagnostics::Diagnostic>,
) -> Result<(), String> {
    for diagnostic in diagnostics {
        eprintln!("{diagnostic}");
    }
    Err(format!("command failed for {path}"))
}

fn usage() -> String {
    "usage: myco <check|inspect> <path>".to_string()
}

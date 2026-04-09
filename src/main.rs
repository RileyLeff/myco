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

            match myco::syntax::parse_and_validate(&source) {
                Ok(model) => match myco::semantic::lower_model(&model) {
                    Ok(semantic) => match myco::equality::lower_model(&semantic) {
                        Ok(equality) => {
                            println!(
                                "ok: model '{}' parsed, validated, and lowered ({} quantities, {} equations, {} slots)",
                                model.name,
                                equality.quantities.len(),
                                equality.equations.len(),
                                equality.slots.len()
                            );
                            Ok(())
                        }
                        Err(diagnostics) => {
                            for diagnostic in diagnostics {
                                eprintln!("{diagnostic}");
                            }
                            Err(format!("check failed for {path}"))
                        }
                    },
                    Err(diagnostics) => {
                        for diagnostic in diagnostics {
                            eprintln!("{diagnostic}");
                        }
                        Err(format!("check failed for {path}"))
                    }
                },
                Err(diagnostics) => {
                    for diagnostic in diagnostics {
                        eprintln!("{diagnostic}");
                    }
                    Err(format!("check failed for {path}"))
                }
            }
        }
        _ => Err(usage()),
    }
}

fn usage() -> String {
    "usage: myco check <path>".to_string()
}

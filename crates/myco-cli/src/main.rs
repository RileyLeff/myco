use std::{env, fs, path::PathBuf, process::ExitCode};

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
                        summary.name,
                        summary.quantity_count,
                        summary.relation_count,
                        summary.slot_count
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
                        "counts: quantities={}, relations={}, slots={}, persistent={}, temporal={}",
                        summary.quantity_count,
                        summary.relation_count,
                        summary.slot_count,
                        summary.persistent_quantity_count,
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
        "emit-demo" => {
            let backend = args.next().ok_or_else(usage)?;
            let path = args.next().ok_or_else(usage)?;
            let output = args.next();
            if args.next().is_some() {
                return Err(usage());
            }

            let backend = parse_backend(&backend)?;
            let source =
                fs::read_to_string(&path).map_err(|err| format!("failed to read {path}: {err}"))?;
            let model = myco_core::pipeline::load_model(&source)
                .map_err(|diagnostics| render_diagnostics(&path, diagnostics))?;
            let artifact = myco_core::pipeline::compile_model(
                &model,
                &myco_core::demo::tiny_tree_training_spec(),
                backend,
            )
            .map_err(|diagnostics| render_diagnostics(&path, diagnostics))?;

            let output_path = output
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from(artifact.suggested_filename()));
            artifact
                .write_to_path(&output_path)
                .map_err(|err| format!("failed to write {}: {err}", output_path.display()))?;
            println!(
                "wrote {} artifact for model '{}' to {}",
                backend_name(backend),
                artifact.model_name,
                output_path.display()
            );
            Ok(())
        }
        "explain-demo" => {
            let path = args.next().ok_or_else(usage)?;
            let quantity = args.next();
            if args.next().is_some() {
                return Err(usage());
            }

            let source =
                fs::read_to_string(&path).map_err(|err| format!("failed to read {path}: {err}"))?;
            let model = myco_core::pipeline::load_model(&source)
                .map_err(|diagnostics| render_diagnostics(&path, diagnostics))?;
            let experiment = myco_core::pipeline::prepare_experiment(
                &model,
                &myco_core::demo::tiny_tree_training_spec(),
            )
            .map_err(|diagnostics| render_diagnostics(&path, diagnostics))?;

            if let Some(quantity) = quantity {
                let explanation = experiment
                    .explain_quantity(&quantity)
                    .map_err(|diagnostics| render_diagnostics(&path, diagnostics))?;
                print_quantity_explanation(&explanation);
            } else {
                let explanation = experiment.explain_plan();
                print_plan_explanation(&explanation);
            }
            Ok(())
        }
        _ => Err(usage()),
    }
}

fn fail_with_diagnostics(
    path: &str,
    diagnostics: Vec<myco_core::diagnostics::Diagnostic>,
) -> Result<(), String> {
    Err(render_diagnostics(path, diagnostics))
}

fn render_diagnostics(path: &str, diagnostics: Vec<myco_core::diagnostics::Diagnostic>) -> String {
    for diagnostic in diagnostics {
        eprintln!("{diagnostic}");
    }
    format!("command failed for {path}")
}

fn parse_backend(value: &str) -> Result<myco_core::pipeline::BackendTarget, String> {
    match value {
        "python" => Ok(myco_core::pipeline::BackendTarget::Python),
        "jax" => Ok(myco_core::pipeline::BackendTarget::Jax),
        _ => Err("backend must be one of: python, jax".to_string()),
    }
}

fn backend_name(value: myco_core::pipeline::BackendTarget) -> &'static str {
    match value {
        myco_core::pipeline::BackendTarget::Python => "python",
        myco_core::pipeline::BackendTarget::Jax => "jax",
    }
}

fn usage() -> String {
    "usage: myco <check|inspect> <path>\n       myco emit-demo <python|jax> <path> [output]\n       myco explain-demo <path> [quantity]"
        .to_string()
}

fn print_plan_explanation(explanation: &myco_core::introspect::PlanExplanation) {
    println!(
        "available current: {}",
        explanation.available_current.join(", ")
    );
    println!("chosen current paths:");
    for path in &explanation.chosen_current {
        println!(
            "  - {} <= {} ({}, cost={}, deps=[{}])",
            path.output,
            path.source,
            path.direction,
            path.cost,
            path.dependencies.join(", ")
        );
    }
    println!("chosen temporal paths:");
    for path in &explanation.chosen_temporal {
        println!(
            "  - {} <= {} ({}, cost={}, deps=[{}])",
            path.output,
            path.source,
            path.direction,
            path.cost,
            path.dependencies.join(", ")
        );
    }
    println!("alternatives:");
    for alternative in &explanation.alternatives {
        println!(
            "  - {} <= {} ({}, cost={})",
            alternative.output, alternative.source, alternative.direction, alternative.cost
        );
    }
    if explanation.unresolved.is_empty() {
        println!("unresolved: none");
    } else {
        println!("unresolved:");
        for unresolved in &explanation.unresolved {
            println!("  - {}", unresolved.quantity);
            for blocked in &unresolved.blocked_candidates {
                println!(
                    "    blocked by {} ({}, cost={}, missing=[{}])",
                    blocked.source,
                    blocked.direction,
                    blocked.cost,
                    blocked.missing_dependencies.join(", ")
                );
            }
        }
    }
}

fn print_quantity_explanation(explanation: &myco_core::introspect::QuantityExplanation) {
    println!("quantity: {}", explanation.quantity);
    println!(
        "direct binding: {}",
        explanation.direct_binding.as_deref().unwrap_or("<none>")
    );
    println!(
        "slot provider: {}",
        explanation.slot_provider.as_deref().unwrap_or("<none>")
    );
    println!("observed: {}", explanation.observed);
    if let Some(path) = &explanation.chosen_current {
        println!(
            "chosen current: {} ({}, cost={}, deps=[{}])",
            path.source,
            path.direction,
            path.cost,
            path.dependencies.join(", ")
        );
    } else {
        println!("chosen current: <none>");
    }
    if let Some(path) = &explanation.chosen_temporal {
        println!(
            "chosen temporal: {} ({}, cost={}, deps=[{}])",
            path.source,
            path.direction,
            path.cost,
            path.dependencies.join(", ")
        );
    } else {
        println!("chosen temporal: <none>");
    }
    if explanation.alternatives.is_empty() {
        println!("alternatives: none");
    } else {
        println!("alternatives:");
        for alternative in &explanation.alternatives {
            println!(
                "  - {} ({}, cost={})",
                alternative.source, alternative.direction, alternative.cost
            );
        }
    }
    if explanation.blocked_candidates.is_empty() {
        println!("blocked candidates: none");
    } else {
        println!("blocked candidates:");
        for blocked in &explanation.blocked_candidates {
            println!(
                "  - {} ({}, cost={}, missing=[{}])",
                blocked.source,
                blocked.direction,
                blocked.cost,
                blocked.missing_dependencies.join(", ")
            );
        }
    }
    println!("unresolved: {}", explanation.unresolved);
}

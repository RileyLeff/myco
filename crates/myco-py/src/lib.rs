use std::{fs, path::PathBuf};

use myco_core::{
    demo,
    diagnostics::Diagnostic,
    pipeline::{
        self, BackendTarget, CompiledArtifact, ExperimentSummary, LoadedModel, ModelSummary,
    },
};
use pyo3::{
    create_exception,
    exceptions::{PyIOError, PyValueError},
    prelude::*,
    types::PyDict,
};

create_exception!(myco_py, MycoError, pyo3::exceptions::PyException);

#[pyfunction]
fn load_model_source(py: Python<'_>, source: &str) -> PyResult<Py<PyDict>> {
    let model = pipeline::load_model(source).map_err(myco_error)?;
    summary_payload(py, &model)
}

#[pyfunction]
fn load_model_path(py: Python<'_>, path: &str) -> PyResult<Py<PyDict>> {
    let source = read_source(path)?;
    load_model_source(py, &source)
}

#[pyfunction]
#[pyo3(signature = (source, backend="jax"))]
fn compile_demo_source(py: Python<'_>, source: &str, backend: &str) -> PyResult<Py<PyDict>> {
    let artifact = compile_demo(source, backend)?;
    let model = pipeline::load_model(source).map_err(myco_error)?;
    let experiment = pipeline::prepare_experiment(&model, &demo::tiny_tree_training_spec())
        .map_err(myco_error)?;

    let payload = PyDict::new(py);
    payload.set_item("model", model_summary_dict(py, &model.summary())?)?;
    payload.set_item(
        "experiment",
        experiment_summary_dict(py, &experiment.summary())?,
    )?;
    payload.set_item("artifact", artifact_dict(py, &artifact)?)?;
    Ok(payload.unbind())
}

#[pyfunction]
#[pyo3(signature = (path, backend="jax"))]
fn compile_demo_path(py: Python<'_>, path: &str, backend: &str) -> PyResult<Py<PyDict>> {
    let source = read_source(path)?;
    compile_demo_source(py, &source, backend)
}

#[pyfunction]
#[pyo3(signature = (path, backend="jax", output_path=None))]
fn write_demo_path(path: &str, backend: &str, output_path: Option<&str>) -> PyResult<String> {
    let source = read_source(path)?;
    let artifact = compile_demo(&source, backend)?;
    let output = output_path
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(artifact.suggested_filename()));
    artifact
        .write_to_path(&output)
        .map_err(|err| PyIOError::new_err(err.to_string()))?;
    Ok(output.display().to_string())
}

#[pymodule]
fn myco_py(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("MycoError", py.get_type::<MycoError>())?;
    m.add_function(wrap_pyfunction!(load_model_source, m)?)?;
    m.add_function(wrap_pyfunction!(load_model_path, m)?)?;
    m.add_function(wrap_pyfunction!(compile_demo_source, m)?)?;
    m.add_function(wrap_pyfunction!(compile_demo_path, m)?)?;
    m.add_function(wrap_pyfunction!(write_demo_path, m)?)?;
    Ok(())
}

fn summary_payload(py: Python<'_>, model: &LoadedModel) -> PyResult<Py<PyDict>> {
    let payload = PyDict::new(py);
    payload.set_item("model", model_summary_dict(py, &model.summary())?)?;
    Ok(payload.unbind())
}

fn compile_demo(source: &str, backend: &str) -> PyResult<CompiledArtifact> {
    let backend = parse_backend(backend)?;
    pipeline::compile_source(source, &demo::tiny_tree_training_spec(), backend).map_err(myco_error)
}

fn parse_backend(name: &str) -> PyResult<BackendTarget> {
    match name.to_ascii_lowercase().as_str() {
        "python" => Ok(BackendTarget::Python),
        "jax" => Ok(BackendTarget::Jax),
        other => Err(PyValueError::new_err(format!(
            "unsupported backend '{other}'; expected 'python' or 'jax'"
        ))),
    }
}

fn read_source(path: &str) -> PyResult<String> {
    fs::read_to_string(path).map_err(|err| PyIOError::new_err(err.to_string()))
}

fn model_summary_dict(py: Python<'_>, summary: &ModelSummary) -> PyResult<Py<PyDict>> {
    let dict = PyDict::new(py);
    dict.set_item("name", &summary.name)?;
    dict.set_item("quantity_count", summary.quantity_count)?;
    dict.set_item("relation_count", summary.relation_count)?;
    dict.set_item("slot_count", summary.slot_count)?;
    dict.set_item("external_count", summary.external_count)?;
    dict.set_item("state_count", summary.state_count)?;
    dict.set_item("node_count", summary.node_count)?;
    dict.set_item("temporal_count", summary.temporal_count)?;
    dict.set_item("quantity_names", &summary.quantity_names)?;
    dict.set_item("relation_names", &summary.relation_names)?;
    dict.set_item("slot_names", &summary.slot_names)?;
    Ok(dict.unbind())
}

fn experiment_summary_dict(py: Python<'_>, summary: &ExperimentSummary) -> PyResult<Py<PyDict>> {
    let dict = PyDict::new(py);
    dict.set_item("name", &summary.name)?;
    dict.set_item("direct_binding_count", summary.direct_binding_count)?;
    dict.set_item("slot_binding_count", summary.slot_binding_count)?;
    dict.set_item("observation_count", summary.observation_count)?;
    dict.set_item("planned_slot_steps", summary.planned_slot_steps)?;
    dict.set_item("planned_equation_steps", summary.planned_equation_steps)?;
    dict.set_item("planned_temporal_steps", summary.planned_temporal_steps)?;
    dict.set_item("alternative_path_count", summary.alternative_path_count)?;
    dict.set_item("unresolved_current_count", summary.unresolved_current_count)?;
    Ok(dict.unbind())
}

fn artifact_dict(py: Python<'_>, artifact: &CompiledArtifact) -> PyResult<Py<PyDict>> {
    let dict = PyDict::new(py);
    dict.set_item("model_name", &artifact.model_name)?;
    dict.set_item("backend", backend_name(artifact.backend))?;
    dict.set_item("suggested_filename", artifact.suggested_filename())?;
    dict.set_item("source", &artifact.source)?;
    Ok(dict.unbind())
}

fn backend_name(backend: BackendTarget) -> &'static str {
    match backend {
        BackendTarget::Python => "python",
        BackendTarget::Jax => "jax",
    }
}

fn myco_error(diagnostics: Vec<Diagnostic>) -> PyErr {
    let message = diagnostics
        .into_iter()
        .map(|diagnostic| diagnostic.to_string())
        .collect::<Vec<_>>()
        .join("\n");
    MycoError::new_err(message)
}

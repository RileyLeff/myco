use std::{fs, path::PathBuf};

use myco_core::{
    compile::{
        CompileMode, CompileSpec, ConsistencyPolicy, DirectBindingKind, DirectBindingSpec,
        InitialStateSource, LossKind, ObservationSchedule, ObservationSpec, SlotBindingKind,
        SlotBindingSpec,
    },
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
    types::{PyAny, PyBool, PyDict, PyList},
};
use serde::Serialize;

create_exception!(_myco_py, MycoError, pyo3::exceptions::PyException);

#[derive(Debug, Serialize)]
struct DiagnosticPayload {
    severity: &'static str,
    message: String,
    span: Option<SourceSpanPayload>,
}

#[derive(Debug, Serialize)]
struct SourceSpanPayload {
    start: SourcePositionPayload,
    end: SourcePositionPayload,
}

#[derive(Debug, Serialize)]
struct SourcePositionPayload {
    line: usize,
    column: usize,
}

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
fn prepare_experiment_source(
    py: Python<'_>,
    source: &str,
    spec: &Bound<'_, PyAny>,
) -> PyResult<Py<PyDict>> {
    let spec = parse_compile_spec(spec)?;
    let model = pipeline::load_model(source).map_err(myco_error)?;
    let experiment = pipeline::prepare_experiment(&model, &spec).map_err(myco_error)?;

    let payload = PyDict::new(py);
    payload.set_item("model", model_summary_dict(py, &model.summary())?)?;
    payload.set_item(
        "experiment",
        experiment_summary_dict(py, &experiment.summary())?,
    )?;
    Ok(payload.unbind())
}

#[pyfunction]
fn prepare_experiment_path(
    py: Python<'_>,
    path: &str,
    spec: &Bound<'_, PyAny>,
) -> PyResult<Py<PyDict>> {
    let source = read_source(path)?;
    prepare_experiment_source(py, &source, spec)
}

#[pyfunction]
fn explain_plan_source(
    py: Python<'_>,
    source: &str,
    spec: &Bound<'_, PyAny>,
) -> PyResult<Py<PyAny>> {
    let spec = parse_compile_spec(spec)?;
    let experiment = prepare_experiment_from_spec(source, &spec)?;
    serialize_to_py(py, &experiment.explain_plan())
}

#[pyfunction]
fn explain_plan_path(py: Python<'_>, path: &str, spec: &Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
    let source = read_source(path)?;
    explain_plan_source(py, &source, spec)
}

#[pyfunction]
fn explain_quantity_source(
    py: Python<'_>,
    source: &str,
    spec: &Bound<'_, PyAny>,
    quantity: &str,
) -> PyResult<Py<PyAny>> {
    let spec = parse_compile_spec(spec)?;
    let experiment = prepare_experiment_from_spec(source, &spec)?;
    let explanation = experiment.explain_quantity(quantity).map_err(myco_error)?;
    serialize_to_py(py, &explanation)
}

#[pyfunction]
fn explain_quantity_path(
    py: Python<'_>,
    path: &str,
    spec: &Bound<'_, PyAny>,
    quantity: &str,
) -> PyResult<Py<PyAny>> {
    let source = read_source(path)?;
    explain_quantity_source(py, &source, spec, quantity)
}

#[pyfunction]
#[pyo3(signature = (source, spec, backend="jax"))]
fn compile_source_with_spec(
    py: Python<'_>,
    source: &str,
    spec: &Bound<'_, PyAny>,
    backend: &str,
) -> PyResult<Py<PyDict>> {
    let spec = parse_compile_spec(spec)?;
    let artifact = compile_with_spec(source, &spec, backend)?;
    let model = pipeline::load_model(source).map_err(myco_error)?;
    let experiment = pipeline::prepare_experiment(&model, &spec).map_err(myco_error)?;

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
#[pyo3(signature = (path, spec, backend="jax"))]
fn compile_path_with_spec(
    py: Python<'_>,
    path: &str,
    spec: &Bound<'_, PyAny>,
    backend: &str,
) -> PyResult<Py<PyDict>> {
    let source = read_source(path)?;
    compile_source_with_spec(py, &source, spec, backend)
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
fn _myco_py(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("MycoError", py.get_type::<MycoError>())?;
    m.add_function(wrap_pyfunction!(load_model_source, m)?)?;
    m.add_function(wrap_pyfunction!(load_model_path, m)?)?;
    m.add_function(wrap_pyfunction!(compile_demo_source, m)?)?;
    m.add_function(wrap_pyfunction!(compile_demo_path, m)?)?;
    m.add_function(wrap_pyfunction!(prepare_experiment_source, m)?)?;
    m.add_function(wrap_pyfunction!(prepare_experiment_path, m)?)?;
    m.add_function(wrap_pyfunction!(explain_plan_source, m)?)?;
    m.add_function(wrap_pyfunction!(explain_plan_path, m)?)?;
    m.add_function(wrap_pyfunction!(explain_quantity_source, m)?)?;
    m.add_function(wrap_pyfunction!(explain_quantity_path, m)?)?;
    m.add_function(wrap_pyfunction!(compile_source_with_spec, m)?)?;
    m.add_function(wrap_pyfunction!(compile_path_with_spec, m)?)?;
    m.add_function(wrap_pyfunction!(write_demo_path, m)?)?;
    Ok(())
}

fn summary_payload(py: Python<'_>, model: &LoadedModel) -> PyResult<Py<PyDict>> {
    let payload = PyDict::new(py);
    payload.set_item("model", model_summary_dict(py, &model.summary())?)?;
    Ok(payload.unbind())
}

fn compile_demo(source: &str, backend: &str) -> PyResult<CompiledArtifact> {
    compile_with_spec(source, &demo::tiny_tree_training_spec(), backend)
}

fn compile_with_spec(
    source: &str,
    spec: &CompileSpec,
    backend: &str,
) -> PyResult<CompiledArtifact> {
    let backend = parse_backend(backend)?;
    pipeline::compile_source(source, spec, backend).map_err(myco_error)
}

fn prepare_experiment_from_spec(
    source: &str,
    spec: &CompileSpec,
) -> PyResult<pipeline::PreparedExperiment> {
    let model = pipeline::load_model(source).map_err(myco_error)?;
    pipeline::prepare_experiment(&model, spec).map_err(myco_error)
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

fn parse_compile_spec(spec: &Bound<'_, PyAny>) -> PyResult<CompileSpec> {
    let dict = spec.downcast::<PyDict>()?;

    Ok(CompileSpec {
        mode: parse_mode(&required_str_item(dict, "mode")?)?,
        horizon_steps: required_extract_item(dict, "horizon_steps")?,
        consistency_policy: parse_consistency_policy(
            optional_str_item(dict, "consistency_policy")?.as_deref(),
        )?,
        direct_bindings: optional_list_items(dict, "direct_bindings")?
            .into_iter()
            .map(parse_direct_binding)
            .collect::<PyResult<Vec<_>>>()?,
        slot_bindings: optional_list_items(dict, "slot_bindings")?
            .into_iter()
            .map(parse_slot_binding)
            .collect::<PyResult<Vec<_>>>()?,
        observations: optional_list_items(dict, "observations")?
            .into_iter()
            .map(parse_observation)
            .collect::<PyResult<Vec<_>>>()?,
    })
}

fn parse_direct_binding(item: Bound<'_, PyAny>) -> PyResult<DirectBindingSpec> {
    let dict = item.downcast::<PyDict>()?;
    let kind = match required_str_item(dict, "kind")?.as_str() {
        "data_series" => DirectBindingKind::DataSeries {
            steps: optional_extract_item(dict, "steps")?.unwrap_or_default(),
        },
        "constant" => DirectBindingKind::Constant,
        "initial_state" => DirectBindingKind::InitialState {
            source: parse_initial_state_source(optional_str_item(dict, "source")?.as_deref())?,
        },
        other => {
            return Err(PyValueError::new_err(format!(
                "unsupported direct binding kind '{other}'"
            )));
        }
    };

    Ok(DirectBindingSpec {
        quantity: required_str_item(dict, "quantity")?,
        kind,
    })
}

fn parse_consistency_policy(value: Option<&str>) -> PyResult<ConsistencyPolicy> {
    match value.unwrap_or("equation_only") {
        "off" => Ok(ConsistencyPolicy::Off),
        "equation_only" => Ok(ConsistencyPolicy::EquationOnly),
        "all" => Ok(ConsistencyPolicy::All),
        other => Err(PyValueError::new_err(format!(
            "unsupported consistency policy '{other}'"
        ))),
    }
}

fn parse_slot_binding(item: Bound<'_, PyAny>) -> PyResult<SlotBindingSpec> {
    let dict = item.downcast::<PyDict>()?;
    let kind = match required_str_item(dict, "kind")?.as_str() {
        "data_series" => SlotBindingKind::DataSeries,
        "constant" => SlotBindingKind::Constant,
        "learned" => SlotBindingKind::Learned,
        other => {
            return Err(PyValueError::new_err(format!(
                "unsupported slot binding kind '{other}'"
            )));
        }
    };

    Ok(SlotBindingSpec {
        slot: required_str_item(dict, "slot")?,
        kind,
    })
}

fn parse_observation(item: Bound<'_, PyAny>) -> PyResult<ObservationSpec> {
    let dict = item.downcast::<PyDict>()?;
    let loss = match required_str_item(dict, "loss")?.as_str() {
        "mse" => LossKind::Mse,
        "huber" => LossKind::Huber,
        other => {
            return Err(PyValueError::new_err(format!(
                "unsupported observation loss '{other}'"
            )));
        }
    };

    let schedule = match required_str_item(dict, "schedule")?.as_str() {
        "dense_per_step" => ObservationSchedule::DensePerStep,
        "sparse" => {
            ObservationSchedule::Sparse(optional_extract_item(dict, "steps")?.unwrap_or_default())
        }
        other => {
            return Err(PyValueError::new_err(format!(
                "unsupported observation schedule '{other}'"
            )));
        }
    };

    Ok(ObservationSpec {
        quantity: required_str_item(dict, "quantity")?,
        loss,
        schedule,
    })
}

fn required_item<'py>(dict: &Bound<'py, PyDict>, key: &str) -> PyResult<Bound<'py, PyAny>> {
    dict.get_item(key)?.ok_or_else(|| {
        PyValueError::new_err(format!("missing required compile-spec field '{key}'"))
    })
}

fn required_extract_item<T>(dict: &Bound<'_, PyDict>, key: &str) -> PyResult<T>
where
    for<'py> T: FromPyObject<'py>,
{
    required_item(dict, key)?.extract()
}

fn optional_extract_item<T>(dict: &Bound<'_, PyDict>, key: &str) -> PyResult<Option<T>>
where
    for<'py> T: FromPyObject<'py>,
{
    match dict.get_item(key)? {
        Some(value) => value.extract().map(Some),
        None => Ok(None),
    }
}

fn required_str_item(dict: &Bound<'_, PyDict>, key: &str) -> PyResult<String> {
    required_extract_item(dict, key)
}

fn optional_str_item(dict: &Bound<'_, PyDict>, key: &str) -> PyResult<Option<String>> {
    optional_extract_item(dict, key)
}

fn optional_list_items<'py>(
    dict: &Bound<'py, PyDict>,
    key: &str,
) -> PyResult<Vec<Bound<'py, PyAny>>> {
    match dict.get_item(key)? {
        Some(value) => value.extract(),
        None => Ok(Vec::new()),
    }
}

fn parse_mode(value: &str) -> PyResult<CompileMode> {
    match value {
        "simulate" => Ok(CompileMode::Simulate),
        "fit" => Ok(CompileMode::Fit),
        "train" => Ok(CompileMode::Train),
        other => Err(PyValueError::new_err(format!(
            "unsupported compile mode '{other}'"
        ))),
    }
}

fn parse_initial_state_source(value: Option<&str>) -> PyResult<InitialStateSource> {
    match value.unwrap_or("constant") {
        "constant" => Ok(InitialStateSource::Constant),
        "data" => Ok(InitialStateSource::Data),
        "learned" => Ok(InitialStateSource::Learned),
        other => Err(PyValueError::new_err(format!(
            "unsupported initial state source '{other}'"
        ))),
    }
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

fn serialize_to_py<T: Serialize>(py: Python<'_>, value: &T) -> PyResult<Py<PyAny>> {
    let value = serde_json::to_value(value)
        .map_err(|err| PyValueError::new_err(format!("failed to serialize payload: {err}")))?;
    json_value_to_py(py, &value)
}

fn json_value_to_py(py: Python<'_>, value: &serde_json::Value) -> PyResult<Py<PyAny>> {
    match value {
        serde_json::Value::Null => Ok(py.None()),
        serde_json::Value::Bool(value) => {
            Ok(PyBool::new(py, *value).to_owned().unbind().into_any())
        }
        serde_json::Value::Number(number) => {
            if let Some(value) = number.as_i64() {
                Ok(value.into_pyobject(py)?.unbind().into_any())
            } else if let Some(value) = number.as_u64() {
                Ok(value.into_pyobject(py)?.unbind().into_any())
            } else if let Some(value) = number.as_f64() {
                Ok(value.into_pyobject(py)?.unbind().into_any())
            } else {
                Err(PyValueError::new_err("unsupported numeric payload"))
            }
        }
        serde_json::Value::String(value) => Ok(value.into_pyobject(py)?.unbind().into_any()),
        serde_json::Value::Array(values) => {
            let items = values
                .iter()
                .map(|item| json_value_to_py(py, item))
                .collect::<PyResult<Vec<_>>>()?;
            let list = PyList::empty(py);
            for item in items {
                list.append(item)?;
            }
            Ok(list.unbind().into_any())
        }
        serde_json::Value::Object(values) => {
            let dict = PyDict::new(py);
            for (key, value) in values {
                dict.set_item(key, json_value_to_py(py, value)?)?;
            }
            Ok(dict.unbind().into_any())
        }
    }
}

fn backend_name(backend: BackendTarget) -> &'static str {
    match backend {
        BackendTarget::Python => "python",
        BackendTarget::Jax => "jax",
    }
}

fn myco_error(diagnostics: Vec<Diagnostic>) -> PyErr {
    let payloads = diagnostics
        .into_iter()
        .map(|diagnostic| DiagnosticPayload {
            severity: match diagnostic.severity {
                myco_core::diagnostics::Severity::Error => "error",
                myco_core::diagnostics::Severity::Warning => "warning",
            },
            message: diagnostic.message,
            span: diagnostic.span.map(|span| SourceSpanPayload {
                start: SourcePositionPayload {
                    line: span.start.line,
                    column: span.start.column,
                },
                end: SourcePositionPayload {
                    line: span.end.line,
                    column: span.end.column,
                },
            }),
        })
        .collect::<Vec<_>>();
    let message = serde_json::to_string(&payloads).unwrap_or_else(|_| "[]".to_string());
    MycoError::new_err(message)
}

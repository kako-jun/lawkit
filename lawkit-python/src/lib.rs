#![allow(clippy::useless_conversion)]

use lawkit_core::{law, LawkitOptions, LawkitResult, LawkitSpecificOptions};
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict};
use serde_json::Value;

/// Convert Python object to serde_json::Value
fn python_to_value(_py: Python, obj: &Bound<'_, PyAny>) -> PyResult<Value> {
    if obj.is_none() {
        Ok(Value::Null)
    } else if let Ok(b) = obj.extract::<bool>() {
        Ok(Value::Bool(b))
    } else if let Ok(i) = obj.extract::<i64>() {
        Ok(Value::Number(serde_json::Number::from(i)))
    } else if let Ok(f) = obj.extract::<f64>() {
        if let Some(n) = serde_json::Number::from_f64(f) {
            Ok(Value::Number(n))
        } else {
            Ok(Value::Null)
        }
    } else if let Ok(s) = obj.extract::<String>() {
        Ok(Value::String(s))
    } else if obj.is_instance_of::<pyo3::types::PyList>() {
        let list = obj.downcast::<pyo3::types::PyList>()?;
        let mut vec = Vec::new();
        for item in list.iter() {
            vec.push(python_to_value(_py, &item)?);
        }
        Ok(Value::Array(vec))
    } else if obj.is_instance_of::<pyo3::types::PyDict>() {
        let dict = obj.downcast::<pyo3::types::PyDict>()?;
        let mut map = serde_json::Map::new();
        for (key, value) in dict.iter() {
            let key_str = key.extract::<String>()?;
            map.insert(key_str, python_to_value(_py, &value)?);
        }
        Ok(Value::Object(map))
    } else {
        // Try to convert to string as fallback
        Ok(Value::String(obj.str()?.extract::<String>()?))
    }
}

/// Convert serde_json::Value to Python object
#[allow(dead_code)]
fn value_to_python(py: Python, value: &Value) -> PyResult<PyObject> {
    match value {
        Value::Null => Ok(py.None()),
        Value::Bool(b) => Ok(b.to_object(py)),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.to_object(py))
            } else if let Some(f) = n.as_f64() {
                Ok(f.to_object(py))
            } else {
                Ok(py.None())
            }
        }
        Value::String(s) => Ok(s.to_object(py)),
        Value::Array(arr) => {
            let py_list = pyo3::types::PyList::empty_bound(py);
            for item in arr {
                py_list.append(value_to_python(py, item)?)?;
            }
            Ok(py_list.to_object(py))
        }
        Value::Object(obj) => {
            let py_dict = pyo3::types::PyDict::new_bound(py);
            for (key, value) in obj {
                py_dict.set_item(key, value_to_python(py, value)?)?;
            }
            Ok(py_dict.to_object(py))
        }
    }
}

/// Convert LawkitResult to Python dictionary
fn lawkit_result_to_python(py: Python, result: &LawkitResult) -> PyResult<PyObject> {
    let dict = pyo3::types::PyDict::new_bound(py);

    match result {
        LawkitResult::BenfordAnalysis(path, data) => {
            dict.set_item("type", "BenfordAnalysis")?;
            dict.set_item("path", path)?;
            dict.set_item(
                "observed_distribution",
                data.observed_distribution.to_object(py),
            )?;
            dict.set_item(
                "expected_distribution",
                data.expected_distribution.to_object(py),
            )?;
            dict.set_item("chi_square", data.chi_square)?;
            dict.set_item("p_value", data.p_value)?;
            dict.set_item("mad", data.mad)?;
            dict.set_item("risk_level", &data.risk_level)?;
            dict.set_item("total_numbers", data.total_numbers)?;
            dict.set_item("analysis_summary", &data.analysis_summary)?;
        }
        LawkitResult::ParetoAnalysis(path, data) => {
            dict.set_item("type", "ParetoAnalysis")?;
            dict.set_item("path", path)?;
            dict.set_item(
                "top_20_percent_contribution",
                data.top_20_percent_contribution,
            )?;
            dict.set_item("pareto_ratio", data.pareto_ratio)?;
            dict.set_item("concentration_index", data.concentration_index)?;
            dict.set_item("risk_level", &data.risk_level)?;
            dict.set_item("total_items", data.total_items)?;
            dict.set_item("analysis_summary", &data.analysis_summary)?;
        }
        LawkitResult::ZipfAnalysis(path, data) => {
            dict.set_item("type", "ZipfAnalysis")?;
            dict.set_item("path", path)?;
            dict.set_item("zipf_coefficient", data.zipf_coefficient)?;
            dict.set_item("correlation_coefficient", data.correlation_coefficient)?;
            dict.set_item("deviation_score", data.deviation_score)?;
            dict.set_item("risk_level", &data.risk_level)?;
            dict.set_item("total_items", data.total_items)?;
            dict.set_item("analysis_summary", &data.analysis_summary)?;
        }
        LawkitResult::NormalAnalysis(path, data) => {
            dict.set_item("type", "NormalAnalysis")?;
            dict.set_item("path", path)?;
            dict.set_item("mean", data.mean)?;
            dict.set_item("std_dev", data.std_dev)?;
            dict.set_item("skewness", data.skewness)?;
            dict.set_item("kurtosis", data.kurtosis)?;
            dict.set_item("normality_test_p", data.normality_test_p)?;
            dict.set_item("risk_level", &data.risk_level)?;
            dict.set_item("total_numbers", data.total_numbers)?;
            dict.set_item("analysis_summary", &data.analysis_summary)?;
        }
        LawkitResult::PoissonAnalysis(path, data) => {
            dict.set_item("type", "PoissonAnalysis")?;
            dict.set_item("path", path)?;
            dict.set_item("lambda", data.lambda)?;
            dict.set_item("variance_ratio", data.variance_ratio)?;
            dict.set_item("poisson_test_p", data.poisson_test_p)?;
            dict.set_item("risk_level", &data.risk_level)?;
            dict.set_item("total_events", data.total_events)?;
            dict.set_item("analysis_summary", &data.analysis_summary)?;
        }
        LawkitResult::IntegrationAnalysis(path, data) => {
            dict.set_item("type", "IntegrationAnalysis")?;
            dict.set_item("path", path)?;
            dict.set_item("laws_analyzed", data.laws_analyzed.to_object(py))?;
            dict.set_item("overall_risk", &data.overall_risk)?;
            dict.set_item(
                "conflicting_results",
                data.conflicting_results.to_object(py),
            )?;
            dict.set_item("recommendations", data.recommendations.to_object(py))?;
            dict.set_item("analysis_summary", &data.analysis_summary)?;
        }
        LawkitResult::ValidationResult(path, data) => {
            dict.set_item("type", "ValidationResult")?;
            dict.set_item("path", path)?;
            dict.set_item("validation_passed", data.validation_passed)?;
            dict.set_item("issues_found", data.issues_found.to_object(py))?;
            dict.set_item("data_quality_score", data.data_quality_score)?;
            dict.set_item("analysis_summary", &data.analysis_summary)?;
        }
        LawkitResult::DiagnosticResult(path, data) => {
            dict.set_item("type", "DiagnosticResult")?;
            dict.set_item("path", path)?;
            dict.set_item("diagnostic_type", &data.diagnostic_type)?;
            dict.set_item("findings", data.findings.to_object(py))?;
            dict.set_item("confidence_level", data.confidence_level)?;
            dict.set_item("analysis_summary", &data.analysis_summary)?;
        }
        LawkitResult::GeneratedData(path, data) => {
            dict.set_item("type", "GeneratedData")?;
            dict.set_item("path", path)?;
            dict.set_item("data_type", &data.data_type)?;
            dict.set_item("count", data.count)?;
            dict.set_item("parameters", data.parameters.to_object(py))?;
            dict.set_item("sample_data", data.sample_data.to_object(py))?;
        }
    }

    Ok(dict.to_object(py))
}

/// Unified law function for Python
///
/// Perform statistical law analysis on Python data using various statistical laws.
///
/// # Arguments
///
/// * `subcommand` - The analysis subcommand ("benf", "pareto", "zipf", "normal", "poisson", "analyze", "validate", "diagnose", "generate")
/// * `data_or_config` - The data to analyze or configuration for generation
/// * `**kwargs` - Optional keyword arguments for configuration
///
/// # Returns
///
/// List of analysis result dictionaries specific to the statistical law used
///
/// # Example
///
/// ```python
/// import lawkit
///
/// # Benford's law analysis
/// data = [123, 456, 789, 1234, 5678]
/// result = lawkit.law("benf", data)
/// print(result)  # [{"type": "BenfordAnalysis", "conformity": 0.85, ...}]
///
/// # Pareto analysis
/// values = [100, 200, 300, 1000, 2000]
/// result = lawkit.law("pareto", values)
/// print(result)  # [{"type": "ParetoAnalysis", "concentration": 0.8, ...}]
/// ```
#[pyfunction(name = "law")]
#[pyo3(signature = (subcommand, data_or_config, **kwargs))]
fn law_py(
    py: Python,
    subcommand: &str,
    data_or_config: &Bound<'_, PyAny>,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<PyObject> {
    // Convert Python objects to serde_json::Value
    let data_value = python_to_value(py, data_or_config)?;

    // Build options from kwargs
    let mut options = LawkitOptions::default();
    let mut lawkit_options = LawkitSpecificOptions::default();
    let mut has_lawkit_options = false;

    if let Some(kwargs) = kwargs {
        for (key, value) in kwargs.iter() {
            let key_str = key.extract::<String>()?;
            match key_str.as_str() {
                // Core options - lawkit doesn't have epsilon or array_id_key
                "ignore_keys_regex" => {
                    if let Ok(pattern) = value.extract::<String>() {
                        options.ignore_keys_regex =
                            Some(regex::Regex::new(&pattern).map_err(|e| {
                                pyo3::exceptions::PyValueError::new_err(format!(
                                    "Invalid regex: {e}"
                                ))
                            })?);
                    }
                }
                "path_filter" => {
                    if let Ok(filter) = value.extract::<String>() {
                        options.path_filter = Some(filter);
                    }
                }
                "output_format" => {
                    if let Ok(format_str) = value.extract::<String>() {
                        let format =
                            lawkit_core::OutputFormat::parse_format(&format_str).map_err(|e| {
                                pyo3::exceptions::PyValueError::new_err(format!(
                                    "Invalid format: {e}"
                                ))
                            })?;
                        options.output_format = Some(format);
                    }
                }
                "show_details" => {
                    if let Ok(show) = value.extract::<bool>() {
                        options.show_details = Some(show);
                    }
                }
                "show_recommendations" => {
                    if let Ok(show) = value.extract::<bool>() {
                        options.show_recommendations = Some(show);
                    }
                }
                "use_memory_optimization" => {
                    if let Ok(opt) = value.extract::<bool>() {
                        options.use_memory_optimization = Some(opt);
                    }
                }
                "batch_size" => {
                    if let Ok(size) = value.extract::<usize>() {
                        options.batch_size = Some(size);
                    }
                }
                // lawkit-specific options - matching actual field names
                "risk_threshold" => {
                    if let Ok(threshold) = value.extract::<String>() {
                        lawkit_options.risk_threshold = Some(threshold);
                        has_lawkit_options = true;
                    }
                }
                "confidence_level" => {
                    if let Ok(level) = value.extract::<f64>() {
                        lawkit_options.confidence_level = Some(level);
                        has_lawkit_options = true;
                    }
                }
                "analysis_threshold" => {
                    if let Ok(threshold) = value.extract::<f64>() {
                        lawkit_options.analysis_threshold = Some(threshold);
                        has_lawkit_options = true;
                    }
                }
                "significance_level" => {
                    if let Ok(level) = value.extract::<f64>() {
                        lawkit_options.significance_level = Some(level);
                        has_lawkit_options = true;
                    }
                }
                "min_sample_size" => {
                    if let Ok(size) = value.extract::<usize>() {
                        lawkit_options.min_sample_size = Some(size);
                        has_lawkit_options = true;
                    }
                }
                "enable_outlier_detection" => {
                    if let Ok(enable) = value.extract::<bool>() {
                        lawkit_options.enable_outlier_detection = Some(enable);
                        has_lawkit_options = true;
                    }
                }
                "enable_japanese_numerals" => {
                    if let Ok(enable) = value.extract::<bool>() {
                        lawkit_options.enable_japanese_numerals = Some(enable);
                        has_lawkit_options = true;
                    }
                }
                "enable_international_numerals" => {
                    if let Ok(enable) = value.extract::<bool>() {
                        lawkit_options.enable_international_numerals = Some(enable);
                        has_lawkit_options = true;
                    }
                }
                "enable_parallel_processing" => {
                    if let Ok(enable) = value.extract::<bool>() {
                        lawkit_options.enable_parallel_processing = Some(enable);
                        has_lawkit_options = true;
                    }
                }
                "memory_limit_mb" => {
                    if let Ok(limit) = value.extract::<usize>() {
                        lawkit_options.memory_limit_mb = Some(limit);
                        has_lawkit_options = true;
                    }
                }
                _ => {
                    // Ignore unknown options
                }
            }
        }
    }

    if has_lawkit_options {
        options.lawkit_options = Some(lawkit_options);
    }

    // Perform law analysis
    let results = law(subcommand, &data_value, Some(&options)).map_err(|e| {
        pyo3::exceptions::PyRuntimeError::new_err(format!("Law analysis error: {e:?}"))
    })?;

    // Convert results to Python objects
    let py_list = pyo3::types::PyList::empty_bound(py);
    for result in results {
        py_list.append(lawkit_result_to_python(py, &result)?)?;
    }

    Ok(py_list.to_object(py))
}

/// A Python module for statistical law analysis toolkit
#[pymodule]
fn lawkit_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(law_py, m)?)?;
    m.add("__version__", "0.2.16")?;
    Ok(())
}

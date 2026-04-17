use pyo3::prelude::*;
use pyo3::types::PyAny;

#[pyclass(module = "kiwisolver._kiwisolver_native")]
pub struct Variable {
    name: String,
    value: f64,
    context: Option<Py<PyAny>>,
}

#[pymethods]
impl Variable {
    #[new]
    #[pyo3(signature = (name="", context=None))]
    fn new(name: &str, context: Option<Py<PyAny>>) -> Self {
        Self {
            name: name.to_owned(),
            value: 0.0,
            context,
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    #[pyo3(name = "setName")]
    fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }

    fn value(&self) -> f64 {
        self.value
    }

    fn context(&self, py: Python<'_>) -> Py<PyAny> {
        self.context
            .as_ref()
            .map(|context| context.clone_ref(py))
            .unwrap_or_else(|| py.None())
    }

    #[pyo3(name = "setContext", signature = (context=None))]
    fn set_context(&mut self, context: Option<Py<PyAny>>) {
        self.context = context;
    }

    fn __repr__(&self) -> String {
        format!("Variable('{}')", self.name)
    }
}

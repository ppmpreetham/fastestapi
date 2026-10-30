use pyo3::prelude::*;

#[pymodule]
mod fastestapi {
    use pyo3::prelude::*;

    /// Stub that prints FastestAPI
    #[pyfunction]
    fn fastestapi() -> PyResult<String> {
        Ok(String::from("FastestAPI"))
    }
}

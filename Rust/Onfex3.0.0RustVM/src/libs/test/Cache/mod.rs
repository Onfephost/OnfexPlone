use crate::ast::*;
use crate::builtins::*;
use crate::error::OnfexError;
use std::collections::HashMap;

use pyo3::prelude::*;
use pyo3::types::PyTuple;


fn 

fn vec_to_py_tuple<'py>(py: Python<'py>,args: Vec<String>,) -> PyResult<Bound<'py, PyTuple>> {
    let py_args: Vec<Bound<'py, PyAny>> = args
        .iter()
        .map(|arg| arg.into_pyobject(py).map(|x| x.into_any()))
        .collect::<PyResult<Vec<_>>>()?;

    PyTuple::new(py, py_args)
}


fn python_run<T>(function: &str,args: Vec<String>,) -> PyResult<T> {
    Python::with_gil(|py| {
        let module = py.import("pycore")?;

        let args = vec_to_py_tuple(py, args)?;

        let result = module
            .getattr(function)?
            .call1(args)?;

        result.extract::<T>()
    })
}


fn run_f64(function: &str,args: Vec<String>,) -> f64 {
    python_run::<f64>(function, args).unwrap()
}


pub fn load_funcs() -> HashMap<String, Fnc> {
    let mut funcs: HashMap<String, Fnc> = HashMap::new();
    //funcs.insert("test".to_string(), test as Fnc);
    funcs
}


pub fn load_vars() -> HashMap<String, Type> {
    let mut vars = HashMap::new();
    let version = "1.0.0".to_string();
    vars.insert("verzen".to_string(), Type::new(TypeKind::Str,Expr::Str(version.clone())));
    vars
}


pub fn load_arrays() -> HashMap<String, ArrayType> {
    let arrays: HashMap<String, ArrayType> = HashMap::new();
    arrays
}


pub fn load_buffers() -> HashMap<String, BufferType> {
    let buffers: HashMap<String, BufferType> = HashMap::new();
    buffers
}


pub fn load_monos() -> HashMap<String, MonoType> {
    let monos: HashMap<String, MonoType> = HashMap::new();
    monos
}
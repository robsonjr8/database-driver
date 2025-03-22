use mysql_async::{params};
use mysql_async::prelude::Queryable;
use pyo3::{prelude::*, wrap_pyfunction};
use serde::Serialize;

#[pyclass(extends=pyo3::exceptions::PyException)]
#[derive(Debug, PartialEq, Eq, Serialize)]
struct Auditdata {
    id: i32,
    name: String,
    age: i32,
}

impl<'py> IntoPyObject<'py> for Auditdata {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        py.get_type::<Auditdata>().call1((self.id, self.age, self.name))
    }
}

#[pymethods]
impl Auditdata {
    #[new]
    fn new(id: i32, age: i32, name: String) -> Auditdata {
        Auditdata { id, name, age }
    }
}

#[pyfunction]
fn execute(py: Python, age: i8) -> PyResult<Bound<PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        let database_url = "mysql://user:password@host:port/db";
        let pool = mysql_async::Pool::new(database_url);
        let mut conn = pool.get_conn().await.expect("mysql_async::Pool::get_conn");
        let data: Vec<_> = conn.exec_map(
            r"SELECT id, name, age
                  FROM person
                  WHERE age >= :age",
            params! { "age" => age },
            |(id, name, age)| {Auditdata { id, name, age }},
        ).await.expect("mysql_async::Params::get");
        drop(conn);
        pool.disconnect().await.expect("mysql_async::Pool::disconnect");
        Ok(data)
    })
}

#[pymodule]
fn db_driver(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(execute, m)?)?;
    Ok(())
}

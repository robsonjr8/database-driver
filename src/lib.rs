use mysql_async::{params, Pool};
use mysql_async::prelude::Queryable;
use pyo3::{prelude::*, wrap_pyfunction};
use serde::Serialize;

#[pyclass(extends=pyo3::exceptions::PyException)]
#[derive(Debug, PartialEq, Eq, Serialize)]
struct Auditdata {
    id: u32,
    name: String,
    age: u8,
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
    fn new(id: u32, age: u8, name: String) -> Auditdata {
        Auditdata { id, name, age }
    }
}

// #[pyfunction]
// fn execute(py: Python, uri: String, age: i8) -> PyResult<Bound<PyAny>> {
//     pyo3_async_runtimes::tokio::future_into_py(py, async move {
//         let opts = mysql_async::Opts::from_url(uri.as_str()).unwrap();
//         let pool = Pool::new(opts);
//         let mut conn = pool.get_conn().await.expect("mysql_async::Pool::get_conn");
//         let data: Vec<_> = conn.exec_map(
//             r"SELECT id, name, age
//                   FROM person
//                   WHERE age >= :age",
//             params! { "age" => age },
//             |(id, name, age)| {Auditdata { id, name, age }},
//         ).await.expect("mysql_async::Params::get");
//         drop(conn);
//         pool.disconnect().await.expect("mysql_async::Pool::disconnect");
//         let json_result = serde_json::to_vec_pretty(&data).unwrap();
//         Ok(json_result)
//     })
// }

#[pyfunction]
fn execute(py: Python, uri: String, age: u8) -> PyResult<Bound<PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        let opts = mysql_async::Opts::from_url(uri.as_str()).unwrap();
        let pool = Pool::new(opts);
        let mut conn = pool.get_conn().await.expect("mysql_async::Pool::get_conn");
        let data: Vec<_> = conn.exec_map(
            "SELECT id, name, age
                  FROM person
                  WHERE age >= :age",
            params! { "age" => age },
            |(id, name, age)| {Auditdata { id, name, age }},
        ).await.expect("mysql_async::Params::get");
        drop(conn);
        pool.disconnect().await.expect("mysql_async::Pool::disconnect");
        let json_result = serde_json::to_vec_pretty(&data).unwrap();
        Ok(json_result)
    })
}

// struct Session {
//     conn: Conn,
//     pool: Pool,
// }
//
// impl<'py> IntoPyObject<'py> for Session {
//     type Target = PyAny;
//     type Output = Bound<'py, Self::Target>;
//     type Error = PyErr;
//
//     fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
//         py.get_type::<Session>().call1((self.conn, self.pool))
//     }
// }
//
// // #[pymethods]
// // impl Session {
// //     #[new]
// //     fn new(conn: Conn, pool: Pool) -> Session {
// //         Session { conn, pool }
// //     }
// // }
//
// trait DbSession {
//     fn new(conn: Conn, pool: Pool) -> Self;
//     async fn execute(self, stmt: &str, age: i32) -> Result<Vec<u8>, ()>;
// }
//
// impl DbSession for Session {
//     fn new(conn: Conn, pool: Pool) -> Session {
//         Session {conn, pool}
//     }
//
//     async fn execute(mut self, stmt: &str, age: i32) -> Result<Vec<u8>, ()> {
//         let data: Vec<Auditdata> = self.conn.exec_map(
//             stmt,
//             params! { "age" => age },
//             |(id, name, age)| {Auditdata { id, name, age }}
//         ).await.expect("mysql_async::Params::get");
//         drop(self.conn);
//         self.pool.disconnect().await.expect("mysql_async::Pool::disconnect");
//         let json_result = serde_json::to_vec_pretty(&data).unwrap();
//         Ok(json_result)
//     }
// }
//
// // async fn query(uri: String, age: i8) -> Result<Vec<u8>, ()> {
// //     let opts = mysql_async::Opts::from_url(uri.as_str()).unwrap();
// //     let pool = mysql_async::Pool::new(opts);
// //     let mut conn = pool.get_conn().await.expect("mysql_async::Pool::get_conn");
// //     let data: Vec<_> = conn.exec_map(
// //         r"SELECT id, name, age
// //               FROM person
// //               WHERE age >= :age",
// //         params! { "age" => age },
// //         |(id, name, age)| {Auditdata { id, name, age }},
// //     ).await.expect("mysql_async::Params::get");
// //     drop(conn);
// //     pool.disconnect().await.expect("mysql_async::Pool::disconnect");
// //     let json_result = serde_json::to_vec_pretty(&data).unwrap();
// //     Ok(json_result)
// // }
//
// #[pyfunction]
// fn session(py: Python, uri: String) -> PyResult<Bound<PyAny>> {
//     pyo3_async_runtimes::tokio::future_into_py(py, async move {
//         let opts = mysql_async::Opts::from_url(uri.as_str()).unwrap();
//         let pool = Pool::new(opts);
//         let conn = pool.get_conn().await.expect("mysql_async::Pool::get_conn");
//         let db: Session = DbSession::new(conn, pool);
//         Ok(db)
//     })
// }

#[pymodule]
fn db_driver(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(execute, m)?)?;
    // m.add_function(wrap_pyfunction!(session, m)?)?;
    Ok(())
}

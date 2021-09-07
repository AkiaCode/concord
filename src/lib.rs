use pyo3::{prelude::*, types::PyDict};
use request::Request;

mod websocket;
mod model;
mod request;


#[pyclass]
struct Client {
    token: String,
}
/**
 * TODO: pyo3_asyncio 를 사용함.
 */
#[pymethods]
impl Client {

    #[new]
    pub fn __new__(token: &str) -> Self {
        Self {
            token: token.to_string(),
        }
    }

    pub fn run(&self, listeners: &PyDict) -> PyResult<()> {

        let websocket = websocket::WebSocketHandler::new(&self.token);
        let mut discord_protcol = websocket.connect();

        Python::with_gil(move |py| {
            ctrlc::set_handler(|| std::process::exit(2)).unwrap();
            loop {
                let d = discord_protcol.recv(py, listeners);
                if d.is_none() { break; }
            }
        });

        Ok(())
    }
}



#[pymodule]
fn concord(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();

    m.add_class::<Client>()?;
    m.add_class::<Request>()?;


    Ok(())
}
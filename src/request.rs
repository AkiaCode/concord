use log::debug;
use pyo3::{exceptions, prelude::*};
use reqwest::{blocking::*, header::USER_AGENT};

#[pyclass]
pub struct Request {
    url: String,
    authorization: String,
    request: Client,
    user_agent: String
}


#[pymethods]
impl Request {
    #[new]
    fn new(token: &str) -> Self {
        Request {
            url: String::from("https://discord.com/api/v8"),
            authorization: String::from(token),
            request: Client::new(),
            user_agent: String::from("DiscordBot (https://github.com/akiacode/concord, v0.0.1)")
        }
    }

    fn post(&self, endpoint: &str, body: &str) -> PyResult<String> {
        let req = self.request.post(self.url.clone() + endpoint)
            .header("Authorization", format!("Bot {}", self.authorization))
            .header("Content-Type", "application/json")
            .header(USER_AGENT, self.user_agent.clone())
            .body(body.to_owned()).send();

        debug!("{:?}", req);
        if let Ok(response) = req {
            Ok(response.text().unwrap())
        } else {
            Err(exceptions::PyException::new_err(req.unwrap_err().to_string()))
        }
    }
}
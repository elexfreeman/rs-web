#![allow(non_snake_case)]

pub mod SampleRouteR {
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Request {
        pub title: String,
        pub description: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Response {
        pub title: String,
        pub description: String,
    }
}

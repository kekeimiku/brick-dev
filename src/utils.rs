// todo!

pub mod sms {
    use crate::error::Result;
    use reqwest::Client;
    use serde::{Deserialize, Serialize};
    use serde_json::from_str;

    #[derive(Serialize)]
    struct Request {}

    #[allow(dead_code)]
    #[derive(Deserialize)]
    struct Response {
        status: String,
    }

    #[allow(unused_variables, unreachable_code)]
    pub async fn send_sms(client: Client, to: &str, code: u32) -> Result<()> {
        let body = todo!();
        let url = "";

        let res = client.post(url).json(&body).send().await?.text().await?;

        if from_str::<Response>(&res)?.status != "success" {
            todo!()
        }

        Ok(())
    }
}

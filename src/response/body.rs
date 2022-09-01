#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AddFileResponseBody {
    pub hash: String,
    pub name: String,
    pub size: String,
}
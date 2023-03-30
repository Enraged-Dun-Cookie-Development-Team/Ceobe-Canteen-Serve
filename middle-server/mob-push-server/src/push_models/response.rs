use crate::MobPushError;

#[derive(Debug, serde::Deserialize)]
pub(crate) struct Respond {
    pub(crate) status: u16,
    #[serde(rename = "res")]
    pub(crate) _res: Option<ResBody>,
    pub(crate) error: Option<String>,
}

impl Respond {
    pub(crate) fn into_result(self) -> Result<Self, MobPushError> {
        match self.status {
            200 => Ok(self),
            state => {
                Err(MobPushError::Mob {
                    state,
                    msg: self.error.unwrap(),
                })
            }
        }
    }
}
#[derive(Debug, serde::Deserialize)]
pub(crate) struct ResBody {
    #[serde(rename = "batchId")]
    pub(crate) _batch_id: String,
}

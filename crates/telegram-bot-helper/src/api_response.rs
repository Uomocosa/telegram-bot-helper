use atomic_delegate_macros::atomic_delegates;
use serde_json::Value;

use crate::Error;

pub struct ApiResponse {
    pub op: &'static str,
    pub status: String,
    pub body: String,
}

#[atomic_delegates]
impl ApiResponse {
    pub fn result_value(&self) -> Result<Value, Error> {}
    pub fn rejection(&self) -> Error {}
    pub fn message_ids(&self) -> Result<Vec<String>, Error> {}
}

#[rustfmt::skip]
impl ApiResponse {
    pub fn message_id(&self) -> Result<String, Error> { Ok(self.result_value()?.get("message_id").and_then(Value::as_u64).map_or_else(|| "ok".to_string(), |id| id.to_string())) }
}

#[cfg(test)]
mod tests {
    use super::ApiResponse;

    fn response(status: &str, body: &str) -> ApiResponse {
        ApiResponse {
            op: "op",
            status: status.to_string(),
            body: body.to_string(),
        }
    }

    #[test]
    fn test_usage() {
        let single = response("200", r#"{"ok":true,"result":{"message_id":42}}"#);
        assert_eq!(single.message_id().unwrap_or_default(), "42");
        let bare = response("200", r#"{"ok":true}"#);
        assert_eq!(bare.message_id().unwrap_or_default(), "ok");
        let many = response(
            "200",
            r#"{"ok":true,"result":[{"message_id":7},{"message_id":8}]}"#,
        );
        assert_eq!(many.message_ids().unwrap_or_default(), vec!["7", "8"]);
        let garbage = response("200", "garbage");
        assert!(garbage.result_value().is_err());
        assert!(garbage.message_id().is_err());
        assert!(garbage.rejection().to_string().contains("status=200"));
        let failed = response("400", r#"{"ok":false,"description":"bad"}"#);
        assert!(failed.message_ids().is_err());
        assert!(failed.rejection().to_string().contains("status=400"));
    }
}

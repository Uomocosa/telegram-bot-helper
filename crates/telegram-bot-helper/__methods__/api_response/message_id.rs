use serde_json::Value;

use crate::ApiResponse;
use crate::Error;

pub fn message_id(response: &ApiResponse) -> Result<String, Error> {
    Ok(response
        .result_value()?
        .get("message_id")
        .and_then(Value::as_u64)
        .map_or_else(|| "ok".to_string(), |id| id.to_string()))
}

#[cfg(test)]
mod tests {
    use super::message_id;
    use crate::ApiResponse;

    #[test]
    fn test_usage() {
        let single = ApiResponse {
            op: "op",
            status: "200".to_string(),
            body: r#"{"ok":true,"result":{"message_id":42}}"#.to_string(),
        };
        assert_eq!(message_id(&single).unwrap_or_default(), "42");
        let bare = ApiResponse {
            op: "op",
            status: "200".to_string(),
            body: r#"{"ok":true}"#.to_string(),
        };
        assert_eq!(message_id(&bare).unwrap_or_default(), "ok");
    }
}

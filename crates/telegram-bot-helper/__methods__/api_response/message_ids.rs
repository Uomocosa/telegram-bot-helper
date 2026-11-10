use serde_json::Value;

use crate::ApiResponse;
use crate::Error;

pub fn message_ids(response: &ApiResponse) -> Result<Vec<String>, Error> {
    let messages = response
        .result_value()?
        .as_array()
        .cloned()
        .ok_or_else(|| response.rejection())?;
    Ok(messages
        .iter()
        .map(|message| {
            message
                .get("message_id")
                .and_then(Value::as_u64)
                .map_or_else(|| "ok".to_string(), |id| id.to_string())
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::message_ids;
    use crate::ApiResponse;

    #[test]
    fn test_usage() {
        let response = ApiResponse {
            op: "op",
            status: "200".to_string(),
            body: r#"{"ok":true,"result":[{"message_id":7},{"message_id":8}]}"#.to_string(),
        };
        assert_eq!(message_ids(&response).unwrap_or_default(), vec!["7", "8"]);
        let not_an_array = ApiResponse {
            op: "op",
            status: "200".to_string(),
            body: r#"{"ok":true,"result":{"message_id":1}}"#.to_string(),
        };
        assert!(message_ids(&not_an_array).is_err());
    }
}

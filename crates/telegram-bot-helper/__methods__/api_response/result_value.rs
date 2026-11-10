use serde_json::Value;

use crate::ApiResponse;
use crate::Error;

pub fn result_value(response: &ApiResponse) -> Result<Value, Error> {
    let parsed: Value = serde_json::from_str(&response.body).map_err(|_| response.rejection())?;
    if !parsed.get("ok").and_then(Value::as_bool).unwrap_or(false) {
        return Err(response.rejection());
    }
    Ok(parsed.get("result").cloned().unwrap_or(Value::Null))
}

#[cfg(test)]
mod tests {
    use super::result_value;
    use crate::ApiResponse;

    #[test]
    fn test_usage() {
        let response = ApiResponse {
            op: "op",
            status: "200".to_string(),
            body: r#"{"ok":true,"result":{"message_id":1}}"#.to_string(),
        };
        assert_eq!(
            result_value(&response)
                .ok()
                .and_then(|value| value.get("message_id").and_then(serde_json::Value::as_u64)),
            Some(1)
        );
    }
}

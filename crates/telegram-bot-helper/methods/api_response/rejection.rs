use crate::ApiResponse;
use crate::Error;

#[must_use]
pub fn rejection(response: &ApiResponse) -> Error {
    let snippet: String = response.body.chars().take(500).collect();
    Error::Rejected {
        op: response.op,
        status: response.status.clone(),
        snippet,
    }
}

#[cfg(test)]
mod tests {
    use super::rejection;
    use crate::ApiResponse;

    #[test]
    fn test_usage() {
        let response = ApiResponse {
            op: "sendMessage",
            status: "400".to_string(),
            body: "bad request".to_string(),
        };
        let message = rejection(&response).to_string();
        assert!(message.contains("status=400"));
        assert!(message.contains("bad request"));
    }
}

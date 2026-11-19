use crate::ApiResponse;

pub fn response(status: &str, body: &str) -> ApiResponse {
    ApiResponse {
        op: "op",
        status: status.to_string(),
        body: body.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::response;

    #[test]
    fn test_usage() {
        assert_eq!(response("200", "{}").status, "200");
    }
}

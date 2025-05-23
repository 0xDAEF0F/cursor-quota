use serde::{Deserialize, Serialize};

// This macro reduces boilerplate code such as defining multiple structs
// https://github.com/yasuyuky/nestruct/blob/main/README.md
nestruct::flatten!(
	#[derive(Debug, Deserialize, Serialize)]
	CursorRes {
		#[serde(rename(deserialize = "gpt-4"))]
		premium_reqs: {
			#[serde(rename = "numRequests")]
			num_requests: u32,
			#[serde(rename = "numRequestsTotal")]
			num_requests_total: u32,
			#[serde(rename = "numTokens")]
			num_tokens: u32,
			#[serde(rename = "maxRequestUsage")]
			max_request_usage: u32,
			#[serde(rename = "maxTokenUsage")]
			max_token_usage: Option<u32>,
		},
		#[serde(rename(deserialize = "gpt-3.5-turbo"))]
		free_reqs: {
			#[serde(rename = "numRequests")]
			num_requests: u32,
			#[serde(rename = "numRequestsTotal")]
			num_requests_total: u32,
			#[serde(rename = "numTokens")]
			num_tokens: u32,
			#[serde(rename = "maxRequestUsage")]
			max_request_usage: Option<u32>,
			#[serde(rename = "maxTokenUsage")]
			max_token_usage: Option<u32>,
		},
		#[serde(rename = "startOfMonth")]
		start_of_month: String,
	}
);

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_cursor_req() {
		let cursor_res = serde_json::from_str::<CursorRes>(
			r#"{
    "gpt-4": {
        "numRequests": 55,
        "numRequestsTotal": 55,
        "numTokens": 41353,
        "maxRequestUsage": 500,
        "maxTokenUsage": null
    },
    "gpt-3.5-turbo": {
        "numRequests": 1,
        "numRequestsTotal": 1,
        "numTokens": 114,
        "maxRequestUsage": null,
        "maxTokenUsage": null
    },
    "gpt-4-32k": {
        "numRequests": 0,
        "numRequestsTotal": 0,
        "numTokens": 0,
        "maxRequestUsage": 50,
        "maxTokenUsage": null
    },
    "startOfMonth": "2025-05-22T19:13:46.000Z"
}"#,
		)
		.unwrap();

		assert_eq!(cursor_res.premium_reqs.num_requests_total, 55);
	}
}

//! Call a llama.cpp server (`llama serve` / `llama-server`).
//!
//! `POST {base}/v1/chat/completions` takes OpenAI-style `messages`.
//! The reply text is `choices[0].message.content`.

use serde::Deserialize;

const CHAT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(60);

/// `{base}` or `{base}/v1` both become `{base}/v1/chat/completions`.
pub fn chat_completions_url(base: &str) -> String {
    let base = base.trim().trim_end_matches('/');
    if base.ends_with("/v1") {
        format!("{base}/chat/completions")
    } else {
        format!("{base}/v1/chat/completions")
    }
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

#[derive(Deserialize)]
struct ChatMessage {
    content: Option<String>,
}

/// Send one prompt and return the assistant text. Non-streaming.
pub async fn chat_completion(base_url: &str, prompt: &str) -> anyhow::Result<String> {
    let url = chat_completions_url(base_url);
    let client = reqwest::Client::builder().timeout(CHAT_TIMEOUT).build()?;
    let body = serde_json::json!({
        "model": "local",
        "messages": [
            {"role": "user", "content": prompt}
        ]
    });
    let resp = client.post(&url).json(&body).send().await?;
    let status = resp.status();
    let text = resp.text().await?;
    if !status.is_success() {
        anyhow::bail!("LLM HTTP {status} from {url}: {text}");
    }
    let parsed: ChatResponse = serde_json::from_str(&text)
        .map_err(|e| anyhow::anyhow!("LLM response from {url} was not chat completions JSON: {e}: {text}"))?;
    let content = parsed
        .choices
        .first()
        .and_then(|choice| choice.message.content.as_deref())
        .unwrap_or("")
        .trim()
        .to_string();
    if content.is_empty() {
        anyhow::bail!("LLM returned an empty reply");
    }
    Ok(content)
}

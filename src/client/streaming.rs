use crate::client::models::StreamEvent;
use crate::error::{AskError, Result};
use futures::stream;
use reqwest::Response;
use tokio_stream::{Stream, StreamExt};

pub fn create_stream(response: Response) -> impl Stream<Item = Result<String>> {
    let byte_stream = response.bytes_stream();

    stream::unfold(
        (byte_stream, String::new()),
        move |(mut stream, mut buffer)| async move {
            while let Some(chunk_result) = stream.next().await {
                match chunk_result {
                    Ok(chunk) => {
                        buffer.push_str(&String::from_utf8_lossy(&chunk));

                        // SSE形式のパースを行う
                        let lines: Vec<&str> = buffer.lines().collect();
                        let mut processed_lines = 0;

                        for (i, line) in lines.iter().enumerate() {
                            if let Some(data) = line.strip_prefix("data: ") {
                                if data == "[DONE]" {
                                    return None; // ストリーム終了
                                }

                                match parse_stream_event(data) {
                                    Ok(Some(text)) => {
                                        // 処理済みの行をバッファから削除
                                        let remaining_lines: Vec<&str> = lines[(i + 1)..].to_vec();
                                        buffer = remaining_lines.join("\n");

                                        return Some((Ok(text), (stream, buffer)));
                                    }
                                    Ok(None) => {
                                        // テキストがない場合は続行
                                        processed_lines = i + 1;
                                        continue;
                                    }
                                    Err(e) => {
                                        return Some((Err(e), (stream, buffer)));
                                    }
                                }
                            }
                            processed_lines = i + 1;
                        }

                        // 処理済みの行をバッファから削除
                        if processed_lines > 0 && processed_lines < lines.len() {
                            let remaining_lines: Vec<&str> = lines[processed_lines..].to_vec();
                            buffer = remaining_lines.join("\n");
                        } else if processed_lines >= lines.len() {
                            buffer.clear();
                        }
                    }
                    Err(e) => {
                        return Some((Err(AskError::NetworkError(e)), (stream, buffer)));
                    }
                }
            }

            None // ストリーム終了
        },
    )
}

fn parse_stream_event(data: &str) -> Result<Option<String>> {
    if data.trim().is_empty() {
        return Ok(None);
    }

    let event: StreamEvent = serde_json::from_str(data)
        .map_err(|e| AskError::StreamError(format!("Failed to parse stream event: {}", e)))?;

    match event.r#type.as_str() {
        "content_block_start" => Ok(None),
        "content_block_delta" => {
            if let Some(delta) = event.delta {
                Ok(Some(delta.text))
            } else {
                Ok(None)
            }
        }
        "content_block_stop" => Ok(None),
        "message_start" => Ok(None),
        "message_delta" => Ok(None),
        "message_stop" => Ok(None),
        "ping" => Ok(None),
        "error" => Err(AskError::StreamError(format!("Stream error: {:?}", event))),
        _ => {
            // 未知のイベントタイプは無視
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_stream_event_delta() {
        let data = r#"{"type":"content_block_delta","index":0,"delta":{"type":"text_delta","text":"Hello"}}"#;
        let result = parse_stream_event(data).unwrap();
        assert_eq!(result, Some("Hello".to_string()));
    }

    #[test]
    fn test_parse_stream_event_start() {
        let data =
            r#"{"type":"content_block_start","index":0,"content_block":{"type":"text","text":""}}"#;
        let result = parse_stream_event(data).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_empty_data() {
        let result = parse_stream_event("").unwrap();
        assert_eq!(result, None);
    }
}

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
                        loop {
                            // 完全なSSEイベントを探す（空行まで）
                            if let Some(event_end) = buffer.find("\n\n") {
                                let event_data = buffer[..event_end].to_string();
                                buffer.drain(..event_end + 2);

                                // データ行を抽出
                                for line in event_data.lines() {
                                    if let Some(data) = line.strip_prefix("data: ") {
                                        if data == "[DONE]" {
                                            return None; // ストリーム終了
                                        }

                                        match parse_stream_event(data) {
                                            Ok(Some(text)) => {
                                                return Some((Ok(text), (stream, buffer)));
                                            }
                                            Ok(None) => {
                                                // テキストがない場合は続行
                                                continue;
                                            }
                                            Err(_) => {
                                                // JSONパースエラーの場合はサイレントにスキップ
                                                continue;
                                            }
                                        }
                                    }
                                }
                            } else {
                                // 完全なイベントがない場合は、次のチャンクを待つ
                                break;
                            }
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

    // JSONパースの前に、データが完全かどうかを簡単にチェック
    let trimmed_data = data.trim();
    if !trimmed_data.starts_with('{') || !trimmed_data.ends_with('}') {
        return Ok(None); // 不完全なJSONはスキップ
    }

    let event: StreamEvent = serde_json::from_str(trimmed_data)
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

    #[test]
    fn test_parse_incomplete_json() {
        let result = parse_stream_event(r#"{"type":"content_block"#);
        assert_eq!(result.unwrap(), None);
    }
}

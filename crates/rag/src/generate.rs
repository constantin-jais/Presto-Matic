//! Grounded question generation: turn a corpus [`Chunk`] into a quiz
//! [`Question`] whose `source_section_ids` cite that chunk. The model is
//! instructed to ground strictly in the source; [`crate::verify`] then checks the
//! result before it is used.

use serde::Deserialize;

use presto_core::protocol::Question;

use crate::corpus::Chunk;
use crate::extract_json;
use crate::provider::{AiError, AiProvider};

const SYSTEM: &str = "You write exactly one multiple-choice quiz question grounded ONLY in the \
    provided source text. Reply with strict JSON: {\"text\": string, \"choices\": array of 4 \
    strings, \"correct_choice\": 0-based integer index}. No prose, no markdown.";

/// A generation failure (provider error or unparseable output).
#[derive(Debug)]
pub struct GenError(pub String);

impl std::fmt::Display for GenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "generation error: {}", self.0)
    }
}

impl std::error::Error for GenError {}

impl From<AiError> for GenError {
    fn from(e: AiError) -> Self {
        GenError(e.to_string())
    }
}

#[derive(Deserialize)]
struct Generated {
    text: String,
    choices: Vec<String>,
    correct_choice: u8,
}

/// Generate one grounded question from a chunk. The returned question cites the
/// chunk via `source_section_ids`, enabling later grounding verification.
pub async fn generate_from_chunk(
    chunk: &Chunk,
    provider: &dyn AiProvider,
) -> Result<Question, GenError> {
    let user = format!("Source:\n{}", chunk.text);
    let raw = provider.complete(SYSTEM, &user).await?;
    let parsed: Generated = serde_json::from_str(extract_json(&raw))
        .map_err(|e| GenError(format!("invalid generation JSON: {e}")))?;

    if parsed.choices.len() < 2 {
        return Err(GenError("a question needs at least two choices".into()));
    }
    if usize::from(parsed.correct_choice) >= parsed.choices.len() {
        return Err(GenError("correct_choice index is out of range".into()));
    }

    Ok(Question {
        id: format!("q:{}", chunk.source_section_id),
        text: parsed.text,
        choices: parsed.choices,
        correct_choice: parsed.correct_choice,
        source_section_ids: vec![chunk.source_section_id.clone()],
        timer_sec: 20,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    struct QuizFake;

    #[async_trait]
    impl AiProvider for QuizFake {
        async fn embed(&self, _texts: &[String]) -> Result<Vec<Vec<f32>>, AiError> {
            Ok(vec![])
        }
        async fn complete(&self, _system: &str, _user: &str) -> Result<String, AiError> {
            // Wrapped in a markdown fence to exercise `extract_json`.
            Ok("```json\n{\"text\":\"What does Rust enforce?\",\
                \"choices\":[\"GC pauses\",\"memory safety\",\"slow builds\",\"nothing\"],\
                \"correct_choice\":1}\n```"
                .to_string())
        }
    }

    #[tokio::test]
    async fn generates_a_question_citing_its_chunk() {
        let chunk = Chunk {
            source_section_id: "doc#p2".into(),
            text: "Rust enforces memory safety without a garbage collector.".into(),
        };
        let q = generate_from_chunk(&chunk, &QuizFake).await.unwrap();
        assert_eq!(q.id, "q:doc#p2");
        assert_eq!(q.source_section_ids, vec!["doc#p2".to_string()]);
        assert_eq!(q.correct_choice, 1);
        assert_eq!(q.choices.len(), 4);
        assert!(q.text.contains("Rust"));
    }

    #[tokio::test]
    async fn rejects_out_of_range_correct_choice() {
        struct BadFake;
        #[async_trait]
        impl AiProvider for BadFake {
            async fn embed(&self, _t: &[String]) -> Result<Vec<Vec<f32>>, AiError> {
                Ok(vec![])
            }
            async fn complete(&self, _s: &str, _u: &str) -> Result<String, AiError> {
                Ok("{\"text\":\"q\",\"choices\":[\"a\",\"b\"],\"correct_choice\":5}".into())
            }
        }
        let chunk = Chunk {
            source_section_id: "d#p0".into(),
            text: "x".into(),
        };
        assert!(generate_from_chunk(&chunk, &BadFake).await.is_err());
    }
}

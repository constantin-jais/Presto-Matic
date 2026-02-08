//! End-to-end WebSocket proof: a host event fans out across connections to a
//! participant, the answer never leaks, and reveal returns a scored leaderboard.

use std::net::SocketAddr;
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

use presto_server::app;
use presto_server::registry::SessionRegistry;

type Ws =
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;

async fn spawn_server() -> SocketAddr {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app(SessionRegistry::new()))
            .await
            .unwrap();
    });
    addr
}

async fn send(ws: &mut Ws, payload: &str) {
    ws.send(Message::text(payload.to_string())).await.unwrap();
}

/// Read frames until one whose `"type"` equals `kind`; panics on timeout.
async fn recv_until(ws: &mut Ws, kind: &str) -> Value {
    let fut = async {
        loop {
            match ws.next().await {
                Some(Ok(Message::Text(t))) => {
                    let v: Value = serde_json::from_str(t.as_str()).unwrap();
                    if v["type"] == kind {
                        return v;
                    }
                }
                Some(Ok(_)) => continue,
                other => panic!("socket closed/error before `{kind}`: {other:?}"),
            }
        }
    };
    tokio::time::timeout(Duration::from_secs(3), fut)
        .await
        .unwrap_or_else(|_| panic!("timed out waiting for `{kind}`"))
}

#[tokio::test]
async fn live_round_fans_out_host_events_to_participants() {
    let addr = spawn_server().await;
    let base = format!("ws://{addr}/ws/sess1");

    let (mut host, _) = connect_async(format!("{base}?role=host&pid=host"))
        .await
        .unwrap();
    let (mut p1, _) = connect_async(format!("{base}?role=participant&pid=p1&name=Alice"))
        .await
        .unwrap();

    // p1 auto-joined; drain the `joined` broadcast.
    recv_until(&mut p1, "joined").await;

    // Host pushes a question — the correct answer must NOT reach participants.
    send(
        &mut host,
        r#"{"type":"push_question","question":{"id":"q1","text":"2+2?","choices":["3","4","5"],"correct_choice":1,"source_section_ids":["doc1#s2"]}}"#,
    )
    .await;
    let q = recv_until(&mut p1, "question_opened").await;
    assert_eq!(q["question"]["id"], "q1");
    assert!(
        q["question"].get("correct_choice").is_none(),
        "the answer leaked to a participant"
    );

    // p1 answers correctly; the host sees an answer_received.
    send(
        &mut p1,
        r#"{"type":"submit_answer","question_id":"q1","choice":1,"elapsed_ms":1200}"#,
    )
    .await;
    let ack = recv_until(&mut host, "answer_received").await;
    assert_eq!(ack["participant_id"], "p1");

    // Host reveals; p1 receives a scored leaderboard + zero confusion (it was right).
    send(&mut host, r#"{"type":"reveal"}"#).await;
    let rev = recv_until(&mut p1, "answers_revealed").await;
    assert_eq!(rev["correct_choice"], 1);
    assert_eq!(rev["leaderboard"][0]["participant_id"], "p1");
    assert!(rev["leaderboard"][0]["score"].as_u64().unwrap() >= 500);
    assert!(rev["heatmap"]["doc1#s2"].as_f64().unwrap().abs() < 1e-6);
}

# packet-macro

패킷 열거형를 쉽게 만들기 위해 제작한 매크로입니다.

## Usage

이렇게 적으면

```rust
#[packet(namespace = "common")]
pub enum CommonPacket {
  #[packet(id = "goto")]
  Goto {
    request: GotoRequestBody,
    response: GotoResponseBody,
  },
  #[packet(id = "died")]
  Died(DiedBody)
}
```

이렇게 나옵니다.

```rust
#[derive(serde::Serialize)]
#[serde(tag = "kind")]
pub enum CommonPacketServer {
  #[serde(rename = "common:goto")]
  Goto {
    ok: bool,
    body: GotoResponseBody,
  },
  #[serde(rename = "common:died")]
  Died {
    ok: bool,
    body: DiedBody
  },
}

#[derive(serde::Deserialize)]
#[serde(tag = "kind")]
pub enum CommonPacketClient {
  #[serde(rename = "common:goto")]
  Goto {
    body: GotoRequestBody
  },
}
```

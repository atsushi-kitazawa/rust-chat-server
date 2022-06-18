## This Repository?

This is a simple chat server written in Rust.

I made it for practicing Rust.

## Usage

Please prepare Rust and Cargo environments before use.

The following is how to execute the program.
```
$ git clone https://github.com/atsushi-kitazawa/rust-chat-server.git
$ cd rust-chat-server
$ cargo run
```

Use a TCP client such as telnet for the client.
```
$ telnet 127.0.0.1 8888
hoge <- type
(127.0.0.1:50670):hoge <- my message
test <- type
(127.0.0.1:50670):test <- my message
(127.0.0.1:50672):hello foo <- another client message
```

## TODO
  - [x] 応答処理の実装
  - [x] 応答処理を別スレッド化し、次の応答に備える
  - [x] 接続したクライアントを保持するマップの実装
    - [x] 接続があったらマップに追加
    - [x] 接続が切れたらマップから削除
  - [x] ブロードキャスト処理
    - [x] マップが保持しているクライアントに送信
  - [ ] チャンネルの実装
#Rustの最新imageを取得
FROM rust:latest 

WORKDIR /app/

COPY . .

#rustupの安定バージョンをインストール
RUN rustup default

#DieselのCLIをインストール
RUN cargo install diesel_cli --no-default-features --features postgres
#cargo-watchをインストール
RUN cargo install cargo-watch

#コンパイラからドキュメントとヒントの表示
CMD ["cargo", "watch", "--why", "--", "echo"]
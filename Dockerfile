#Rustの最新imageを取得
FROM rust:latest 

WORKDIR /app/

COPY Cargo.toml Cargo.toml

RUN mkdir src
RUN echo "fn main(){}" > src/main.rust

RUN cargo build --release

COPY . .

#rustupの安定バージョンをインストール
RUN rustup default

RUN rm -f target/release/dops/app*

RUN cargo build --release

#DieselのCLIをインストール
RUN cargo install diesel_cli --no-default-features --features postgres
#cargo-watchをインストール
RUN cargo install cargo-watch

#コンパイラからドキュメントとヒントの表示
CMD ["cargo", "watch", "--why", "--", "echo"]
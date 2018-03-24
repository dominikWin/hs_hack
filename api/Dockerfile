FROM rustlang/rust:nightly

WORKDIR /usr/src/api
COPY . .

RUN cargo install

EXPOSE 8000

CMD ["api"]

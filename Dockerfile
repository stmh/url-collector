FROM rust:1.92 as builder

WORKDIR ./app
ADD . ./
RUN cargo build --release

FROM rust:1.92

# copy the build artifact from the build stage
COPY --from=builder /app/target/release/url-collector /usr/local/bin

# set the startup command to run your binary
CMD ["/usr/local/bin/url-collector"]

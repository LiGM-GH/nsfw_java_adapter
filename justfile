set dotenv-load

default:
    just --list

clean:
    bash -c 'if [[ -f "./target/debug/libnsfw_lib.so" ]]; then echo "Found libfile, cleaning..."; rm "./target/debug/libnsfw_lib.so"; echo "Clean!"; fi'

build:
    cargo build

run: build
    LD_LIBRARY_PATH="`pwd`/target/debug" java ./java_code/NsfwPredictor.java

rerun: clean build run

test:
    cargo test -- --nocapture

daemon:
    cargo run --bin nsfw_daemon

java: build
    LD_LIBRARY_PATH="`pwd`/target/debug" java ./java_code/NsfwPredictor.java $IMAGE_TO_PROCESS

release-clean:
    bash -c 'if [[ -f "./target/release/libnsfw_lib.so" ]]; then echo "Found libfile, cleaning..."; rm "./target/release/libnsfw_lib.so"; echo "Clean!"; fi'

release-build:
    cargo build --release

release-run: release-build
    LD_LIBRARY_PATH="`pwd`/target/release" java ./java_code/NsfwPredictor.java

release-rerun: release-clean release-build release-run

release-test:
    cargo test --release -- --nocapture

release-daemon:
    cargo run --release --bin nsfw_daemon

release-java: release-build
    LD_LIBRARY_PATH="`pwd`/target/release" java ./java_code/NsfwPredictor.java $IMAGE_TO_PROCESS

default:
    echo 'Hello, world!'

build:
    cargo build

clean:
    bash -c 'if [[ -f "./target/debug/libnsfw_lib.so" ]]; then echo "Found libfile, cleaning..."; rm "./target/debug/libnsfw_lib.so"; echo "Clean!"; fi'

run: build
    LD_LIBRARY_PATH="`pwd`/target/debug" java ./java_code/NsfwPredictor.java

rerun: clean build run

test:
    cargo test -- --nocapture

release-build:
    cargo build --release

release-run: release-build
    LD_LIBRARY_PATH="`pwd`/target/release" java ./java_code/NsfwPredictor.java

release-test:
    cargo test --release -- --nocapture

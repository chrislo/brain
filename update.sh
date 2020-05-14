cd "$(dirname "$0")"

git pull &&
    cd sequencer && cargo build --release && cd -

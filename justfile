default:
    just --list
setup day:
    cargo scaffold {{day}} --download
test day:
    cargo test --bin {{day}}
debug day:
    cargo solve {{day}}
solve day:
    cargo solve {{day}} --release
submit day part:
    cargo solve {{day}} --release --submit {{part}}

default:
    just --list
setup day:
    cargo scaffold {{day}} --download
solve day:
    cargo solve {{day}}
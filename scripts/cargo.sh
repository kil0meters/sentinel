#!/bin/sh

export CARGO_HOME=$1/target/cargo-home

if [[ $DEBUG = true ]]
then
    echo "DEBUG MODE"
    cargo build -p sentinel-gtk && cp $1/target/debug/sentinel-gtk $2
else
    echo "RELEASE MODE"
    cargo build --release -p sentinel-gtk && cp $1/target/release/sentinel-gtk $2
fi

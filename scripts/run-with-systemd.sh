#!/bin/bash
su root
export CARGO_HOME="/home/sean/.cargo"
make $1

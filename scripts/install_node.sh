#!/bin/sh

cargo install --path src/bin

cardano node install -vv -y

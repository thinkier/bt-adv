#!/bin/bash

docker build -f aarch64.Dockerfile -t ghcr.io/thinkier/bt-adv-aarch64-unknown-linux-gnu .
docker build -f x86_64.Dockerfile -t ghcr.io/thinkier/bt-adv-x86_64-unknown-linux-gnu .

docker push ghcr.io/thinkier/bt-adv-aarch64-unknown-linux-gnu
docker push ghcr.io/thinkier/bt-adv-x86_64-unknown-linux-gnu

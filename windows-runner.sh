#!/bin/sh
# windows-runner.sh
# This script starts the docker container, adding our required security option.
# It then passes along all the other arguments from cross.

exec docker run --security-opt seccomp=unconfined "$@"
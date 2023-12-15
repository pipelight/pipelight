#! /usr/bin/env bash


pipelight logs rm
pipelight run test_dummy_cmd
vhs tapes/logs.tape

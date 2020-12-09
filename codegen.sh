#!/bin/bash

SCHEMA_PATH="schema/model.fbs"

echo "Generating Rust Code: for ${PWD}/${SCHEMA_PATH}"
flatc --rust -o src/generated ${SCHEMA_PATH}

echo "Generating Python Code: for ${PWD}/${SCHEMA_PATH}"
flatc --python -o python/generated ${SCHEMA_PATH}
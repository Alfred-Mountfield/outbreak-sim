#!/bin/bash

SCHEMA_PATH="schema/model.fbs"

echo "Clearing existing generated code in ${PWD}/src/generated"
rm -r src/generated

echo "Clearing existing generated code in ${PWD}/python/generated"
rm -r python/generated

echo "Generating Rust Code: for ${PWD}/${SCHEMA_PATH}"
flatc --rust -o src/generated ${SCHEMA_PATH}

echo "Generating Python Code: for ${PWD}/${SCHEMA_PATH}"
flatc --python -o python/generated ${SCHEMA_PATH}
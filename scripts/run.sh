#!/bin/bash

# Function to execute the first curl command
transform() {
  DataXml="examples/data.xml"
  curl -X POST http://127.0.0.1:8080/transform \
    -H "Content-Type: application/xml" \
    -d "@$DataXml"
}

validate() {
  DataXml="examples/data.xml"
  curl -X POST http://127.0.0.1:8080/validate \
    -H "Content-Type: application/xml" \
    -d "@$DataXml"
}

# Check if a command line argument is provided
if [ $# -eq 0 ]; then
  echo "No arguments provided. Use --transform or --validate"
  exit 1
fi

# Handle command line arguments
case $1 in
--transform)
  transform
  ;;
--validate)
  validate
  ;;
*)
  echo "Invalid option: $1.  Use --transform or --validate"
  exit 1
  ;;
esac

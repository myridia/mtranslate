#!/bin/bash
echo "start test2"
html_content=$(cat body.html)
curl -X POST 'http://127.0.0.1:8089/' \
     -H 'Content-Type: application/json' \
     -d "$(jq -n --arg html "$html_content" '{"html":$html,"t":"de","s":"sv"}' )"




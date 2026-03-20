#!/bin/bash

echo "start test2"
html_content=$(cat body.html)
#echo $html_content
#curl -X POST  'http://127.0.0.1:8089/'  -H 'Content-Type:application/json' -d '{"html":@body.html,"t":"da","s":"sv"}'
curl -X POST 'http://127.0.0.1:8089/' \
     -H 'Content-Type: application/json' \
     -d "$(jq -n --arg html "$html_content" '{"html":$html,"t":"de","s":"sv"}' )"




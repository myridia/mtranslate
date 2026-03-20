#!/bin/bash
echo "start test2"
html_content=$(cat body.html)
curl -X POST 'https://mtranslate.myridia.com/' \
     -H 'Content-Type: application/json' \
     -d "$(jq -n --arg html "$html_content" '{"html":$html,"t":"en","s":"sv"}' )"




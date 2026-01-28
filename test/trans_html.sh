#!/bin/bash


#curl -X GET 'http://127.0.0.1:8089?s=sv&t=de&v=Dessa'


#curl -X POST  http://0.0.0.0:8089/translate_html  -H 'Content-Type:application/json' -d '{"html":"<div class="\hello should not be translated\" >hello</div>","t":"ru","s":"en"}'
#curl -X POST  http://0.0.0.0:8089/translate_html  -H 'Content-Type:application/json' -d '{ "html":"<div>hello</div>","s":"en","t":"ru" }'
#curl -X POST  http://127.0.0.1:8089/translate_html  -H 'Content-Type:application/json' -d '{"html":"<a href=\"https://www.app.local/kategori/herr/klader-herr/t-shirts-herr/\"><img class=\"alignnone wp-image-643788 size-full\" src=\"https://www.app.local/wp-content/uploads/2018/03/2-delad-merch.jpg\" alt=\"\" width=\"667\" height=\"800\" />Dessa</a>","t":"da","s":"sv"}'
curl -X POST  http://127.0.0.1:8089/translate_html  -H 'Content-Type:application/json' -d '{"html":"<a href=\"https://www.app.local/kategori/herr/klader-herr/t-shirts-herr/\"><img class=\"alignnone wp-image-643788 size-full\" src=\"https://www.app.local/wp-content/uploads/2018/03/2-delad-merch.jpg\" alt=\"\" width=\"667\" height=\"800\" />Dessa</a>","t":"da","s":"sv"}'
#curl -X POST  https://mtranslate.myridia.com/translate_html  -H 'Content-Type:application/json' -d '{"html":"<a href=\"https://www.app.local/kategori/herr/klader-herr/t-shirts-herr/\"><img class=\"alignnone wp-image-643788 size-full\" src=\"https://www.app.local/wp-content/uploads/2018/03/2-delad-merch.jpg\" alt=\"\" width=\"667\" height=\"800\" />Dessa</a>","t":"da","s":"sv"}'



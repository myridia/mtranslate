DB_NAME=dbsql1
DB_USER=dbsql1
DB_PASSWORD=passpass


echo -e "I'm ask.sh. What you like to do?, enter a Task Id from list below: \n"
echo -e "TaskID\t Description"
echo -e "1\t Test"
echo -e "2\t Post translate_html "




until [ "$task" = "0" ]; do
read task

if [ "$task" = "1" ]; then
    echo "...${task}"
     curl -X GET 'http://127.0.0.1:8089/test'
    
elif [ "$task" = "2" ]; then
    echo "...${task}"
    curl -X POST  http://127.0.0.1:8089/translate_html  -H 'Content-Type:application/json' -d '{"html":"<a href=\"https://www.app.local/kategori/herr/klader-herr/t-shirts-herr/\"><img class=\"alignnone wp-image-643788 size-full\" src=\"https://www.app.local/wp-content/uploads/2018/03/2-delad-merch.jpg\" alt=\"\" width=\"667\" height=\"800\" />Dessa</a>","t":"da","s":"sv"}'    


    
else
    echo "Goodbye! - Exit"
fi


sleep 3
./ask.sh

done 

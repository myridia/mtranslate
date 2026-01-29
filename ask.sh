DB_NAME=dbsql1
DB_USER=dbsql1
DB_PASSWORD=passpass


echo -e "I'm ask.sh. What you like to do?, enter a Task Id from list below: \n"
echo -e "TaskID\t Description"
echo -e "1\t Run - Docker Test Enviroment and API"
echo -e "2\t Run - Docker Page "
echo -e "3\t Clean Docker - Clean the docker containers and volumes "
echo -e "4\t Clean All - Clean the docker containers and volumes and images "
echo -e "5\t Export Db - Export the database on the docker/test server"
echo -e "6\t Test html text via curl LOCAL"
echo -e "7\t Test html text via curl ONLINE "
echo -e "8\t Test plain text via curl "



until [ "$task" = "0" ]; do
read task

if [ "$task" = "1" ]; then
    echo "...${task}"
    cd dockers
    docker-compose up -d
    echo "API:"
    echo "http://127.0.0.1:5800"
    echo "phpmyadmin"
    echo "http://127.0.0.1:81/"        
    cd ../
    make
    
elif [ "$task" = "2" ]; then
    echo "... ${task} -- Run Docker Page"
    cd pages/dockers
    docker-compose up -d    
    echo "Visit:"
    echo "http://127.0.0.1:88"    

    
elif [ "$task" = "3" ]; then
    echo "...${task}"
    docker stop `docker ps -qa`        
    docker rm `docker ps -qa`    
    docker volume rm $(docker volume ls -q --filter dangling=true)
    docker network prune --force
    
elif [ "$task" = "4" ]; then
    echo "...${task}"
    docker stop `docker ps -qa`            
    docker rm  `docker ps -qa`
    docker volume rm $(docker volume ls -q --filter dangling=true)
    docker network prune
    docker rmi --force `docker images -aq`    

elif [ "$task" = "5" ]; then
    echo "...${task}"
    docker  run -i --rm --net=dockers_workgroup  salamander1/mysqldump --verbose -h "10.5.0.2" -u "${DB_NAME}" -p"${DB_PASSWORD}"  "${DB_NAME}" | gzip > "dockers/init/${DB_NAME}.sql.gz"


elif [ "$task" = "6" ]; then
    echo "...${task}"
    curl -X POST  http://0.0.0.0:8089/translate_html -H 'Content-Type:application/json'  -d '{"html":"<div class=\"hello\">Hello world</div>","t":"ru","s":"en"}' 


elif [ "$task" = "7" ]; then
    echo "...${task}"
    curl -X POST  https://mtranslate.myridia.com/translate_html -H 'Content-Type:application/json'  -d '{"html":"<div class=\"hello\">Hello world</div>","t":"ru","s":"en"}' 
    

elif [ "$task" = "8" ]; then
    echo "...${task}"
    curl -X GET 'http://0.0.0.0:8089?s=en&t=th&v=hello'

    
else
    echo "Goodbye! - Exit"
fi


sleep 3
./ask.sh

done 

DB_NAME=dbsql1
DB_USER=dbsql1
DB_PASSWORD=passpass
DATE=$(date +"%F")


echo -e "I'm ask.sh. What you like to do?, enter a Task Id from list below: \n"
echo -e "TaskID\t Description"
echo -e "1\t Run - Docker Test Enviroment "
echo -e "2\t Run - Docker Page "
echo -e "3\t Clean Docker - Clean the docker containers and volumes "
echo -e "4\t Clean All - Clean the docker containers and volumes and images "
echo -e "5\t Export Db - Export the database on the docker/test server"




read task

if [ "$task" = "1" ]; then
    echo "...${task}"
    cd dockers
    docker-compose up -d
    echo "API:"
    echo "http://127.0.0.1:5800"
    echo "phpmyadmin"
    echo "http://127.0.0.1:81/"        

    
    
elif [ "$task" = "2" ]; then
    echo "... ${task} -- Run Docker Page"
    cd pages/dockers
    docker-compose up -d    
    echo "Visit:"
    echo "http://127.0.0.1:88"    

    
elif [ "$task" = "3" ]; then
    echo "...${task}"    
    docker rm `docker ps -qa`
    docker volume rm $(docker volume ls -q --force --filter dangling=true)
    docker network prune --force
    
elif [ "$task" = "4" ]; then
    echo "...${task}"
    docker rm --force `docker ps -qa`
    docker volume rm $(docker volume ls -q --filter dangling=true)
    docker network prune
    docker rmi --force `docker images -aq`    

elif [ "$task" = "5" ]; then
    echo "...${task}"
    docker  run -i --rm --net=host  salamander1/mysqldump --verbose -h db -u "${DB_NAME}" -p"${DB_PASSWORD}"  "${DB_NAME}" | gzip > "./test/init/${DB_NAME}-${DATE}.sql.gz"
    docker  run -i --rm --net=host  salamander1/mysqldump --verbose -h db -u "${DB_NAME}" -p"${DB_PASSWORD}"  "${DB_NAME}" | gzip > "./test/init/${DB_NAME}.sql.gz"

    
else
    echo "Goodbye! - Exit"
fi



## How to run the server on local?
Step 1: setup database:
    - mysql server:
        > docker-compose up --build -d
    - initialize migrations:
        > sqlx migrate run 
Step 2: run server:
    > cargo run

## How to access server?
    - use requests.http file inside your IDE
    - 
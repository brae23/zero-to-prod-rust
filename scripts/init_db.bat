set "DB_USER=postgres"
set "DB_PASSWORD=password"
set "DB_NAME=newsletter"
set "DB_PORT=5432"
set "DB_HOST=localhost"
docker run -e POSTGRES_USER=%DB_USER% -e POSTGRES_PASSWORD=%DB_PASSWORD% -e POSTGRES_DB=%DB_NAME% -p %DB_PORT% -d postgres postgres -N 1000
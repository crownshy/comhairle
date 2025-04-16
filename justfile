dev:
    cd api && bacon

pg:
    docker run -it --rm --name comhairle_postgres \
    -p 5434:5432 \
    -e POSTGRES_USER=comhairle \
    -e POSTGRES_PASSWORD=comhairle \
    -e POSTGRES_DB=comhairle \
    -v $(pwd)/pg_data:/var/lib/postgresql/data \
    postgres

psql:
    psql -U comhairle -d comhairle  -h localhost -p 5434

load_saia:
    cargo run --bin comhairle_data_loader -- -f fixtures/saia.json -d true

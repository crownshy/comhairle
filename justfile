dev:
    cd api && bacon

pg:
    docker run -it --rm --name comhairle_postgres \
    -p 5434:5432 \
    -e POSTGRES_USER=comhairle \
    -e POSTGRES_PASSWORD=comhairle \
    -e POSTGRES_DB=comhairle \
    -v $(pwd)/pg_data:/var/lib/postgresql/data \
    postgres:16

psql:
    psql -U comhairle -d comhairle  -h localhost -p 5434

load_saia:
    cargo run --bin comhairle_data_loader -- -f fixtures/saia.json -d true

api-dev:
    cargo watch -q -c \
    -i open-api-spec.json \
    -w api/src/ \
    -w adaptors \
    -x "run -- --export-api-spec"

watch-api-spec:
    watchexec -d 3s -w open-api-spec.json -- pnpm --dir ui run client

api-watch:
    just api-dev & just watch-api-spec

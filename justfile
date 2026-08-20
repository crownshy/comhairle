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

generate-api-spec:
    cargo run --bin api_spec_gen --

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

# Create admin user (requires API running)
seed:
    ./scripts/seed-minimal.sh

# Start Redis for the apalis worker service (creates container on first run)
start-redis:
    docker start apalis-redis 2>/dev/null || docker run -d \
        --name apalis-redis \
        -p 63793:6379 \
        -v apalis-redis-data:/data \
        redis:7 redis-server --save 60 1

# Run DB, Redis, API and UI in separate tmux windows
all:
    just start-redis
    tmux new-session -d -s comhairle -n postgres "just pg" \; \
        new-window -n api "just api-dev" \; \
        new-window -n ui "cd ui/packages && pnpm comhairle" \; \
        new-window -n seed "echo 'wait for api then: just seed'; exec $SHELL" \; \
        attach-session -t comhairle

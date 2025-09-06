set shell := ["bash", "-cu"]

frontend:
    cd frontend && pnpm dev

backend:
    sudo -v
    sudo kubefwd services -n default > /dev/null &
    cd backend && cargo run --bin api

migrate:
    sqlx database reset -y --database-url postgres://postgres@localhost:5432/postgres

clean-nodes:
    kubectl get deployments --no-headers -o custom-columns=":metadata.name" | grep '^anvil-' | xargs -r kubectl delete deployment
    kubectl get services --no-headers -o custom-columns=":metadata.name" | grep '^anvil-' | xargs -r kubectl delete service

setup:
    @just start-minikube
    docker run -d --env POSTGRES_HOST_AUTH_METHOD=trust -p 5432:5432 --name postgres postgres:16-alpine
    @just migrate

start-minikube:
    minikube start -d docker --embed-certs

start:
    @just start-minikube
    docker start postgres

stop:
    minikube stop
    docker stop postgres

delete:
    minikube delete
    docker stop postgres
    docker rm postgres

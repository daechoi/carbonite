POSTGRES_PASSWORD := 'postgres'
DATABASE_URL := 'postgres://postgres:postgres@zerodb:5432'
CONDUCTOR_DATABASE_URL := 'postgresql://postgres:postgres@0.0.0.0:5431/postgres'
CLERK_SECRET_KEY := 'postgres-secret-key'
RUST_LOG := 'info'
KUBE_VERSION := '1.30'
CERT_MANAGER_VERSION := '1.13.2'

run-dbs:
  docker network create zero2prodnet || true
  docker rm --force zerodb || true
  docker run --network zero2prodnet -d --name zerodb -e POSTGRES_PASSWORD={{POSTGRES_PASSWORD}}  -p 5431:5432 postgres

dbs-start:
  just run-dbs

dbs-cleanup:
  docker stop cp-pgmq-pg 

helm-lint:
  ct lint --config ct.yaml 

helm-repo:
  helm repo add cpng https://cloudnative-pg.github.io/charts

cert-manager:
  helm repo add jetstack https://charts.jetstack.io && helm repo update
  helm upgrade --install cert-manager jetstack/cert-manager --version={{CERT_MANAGER_VERSION}} --namespace cert-manager --create-namespace
  sleep 5 
  kubectl wait --timeout=120s --for=condition=Ready pod -l app.kubernetes.io/name=cert-manager -n cert-manager

start-kind:
  kind delete cluster 
  kind create cluster --config k8s/kind-{{KUBE_VERSION}}.yaml
  sleep 5 
  kubectl wait --for=condition=Ready pod --all --all-namespaces --timeout=300s
#  just cert-manager

watch-operator:
    docker container rm kembo-control-plane --force || true
    docker container rm kembo-worker --force || true
    just -f ./kembo-operator/justfile start-kind
    DATA_PLANE_BASEDOMAIN=local.kembo-development.com \
    ENABLE_BACKUP=false RUST_LOG=info,kube=info,controller=info \
    PORT=6000 \
    cargo watch --workdir ./tembo-operator -x 'run'



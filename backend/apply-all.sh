kubectl apply -f charts/sealed-secrets.local.yaml

kubectl apply -f charts/kubeconfig.secret.yaml

kubectl apply -f charts/postgres.secret.yaml \
    -f charts/postgres.configmap.yaml \
    -f charts/postgres.volume.yaml \
    -f charts/postgres.deployment.yaml \
    -f charts/postgres.service.yaml

kubectl apply -f charts/api.secret.yaml \
    -f charts/api.configmap.yaml \
    -f charts/api.deployment.yaml \
    -f charts/api.service.yaml \
    -f charts/api.ingress.yaml

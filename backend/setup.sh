# Check if the DATABASE_URL environment variable is set
: ${DATABASE_URL?}

# Setup minikube and apply all kubernetes resources
minikube start -d docker --embed-certs
minikube addons enable ingress
kubectl apply -f https://github.com/bitnami-labs/sealed-secrets/releases/download/v0.28.0/controller.yaml

# Apply all kubernetes resources of the project
./apply-all.sh

# Migrate the database
pod=$(kubectl get pods -l app.kubernetes.io/name=postgres | cut -f 1 -d ' ' | tail -n 1)
kubectl port-forward $pod 5432:5432 &
pid=$!
sqlx database reset -y
kill $pid

# Start the minikube tunnel
minikube tunnel

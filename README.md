## Setup

You just need to run this command to setup the project:

```bash
just setup
```

This will setup minikube, create a container with a postgres database and apply the migration.

## Run project

### Frontend

To run only the frontend just run:

```bash
just frontend
```

### Backend

To run only the backend just run:

```bash
just backend
```

## Stop

To stop minikube, and the postgres container just run:

```bash
just stop
```

## Start

To restart minikube and the postgres container just run:

```bash
just start
```

## Delete

To delete minikube and the postgres container just run:

```bash
just delete
```

## Utils commands

### Migrate

To reset and apply the migration just run:

```bash
just migrate
```

### Remove all nodes pods

To remove all nodes pods just run:

```bash
just clean-nodes
```

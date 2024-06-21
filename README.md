# Kubernetes project : ara-monitoring

## Prerequisites
- Kubernetes cluster (k3d/k3s, rke2, ...)
- A storage class (local-path, longhorn, ...)
- A secret containing the informations to access to our registry. You can generate one with the command below :
```bash
kubectl create secret docker-registry regcred -n ara-back \
  --docker-server=<registry> \
  --docker-username=<username> \
  --docker-password=<password> \
  --docker-email=<email>
```

PS : We can also install ArgoCD to manage the deployment of the project more easily like I did. You can find the installation steps [here](https://argoproj.github.io/argo-cd/getting_started/).

## Global overview

The project is splitted in three main branches :
- **production** : The branch that is used to deploy the project in a Kubernetes cluster WITHOUT the migration feature
- **feat/db_migrations** : The branch that is used to deploy the project in a Kubernetes cluster WITH the migration feature
- **feat/ara-monitoring-wasi** : A POC using Web Assembly to compile and run the API (see the README in the branch)

The project is composed of two main services :
- **api** : an classic API using Express, a Javascript framework
- **db** : yet another Postgresql database

## API

### Env

- `POSTGRES_USER`: Username of the user in the postgresql database
- `POSTGRES_PASSWORD`: Password of the user in the postgresql database
- `POSTGRES_HOSTNAME`: Hostname of the machine running postgresql
- `POSTGRES_DB`: Name of the database to connect to

### Routes

Here is a list of the accessible routes :
- **GET /** : Return 200 - OK
-  **GET /healthz** : Call the database for date and time
- **GET /users** : Return all users in the database
- **POST /adduser** : Create a user in the database like this : 
```json
{
    "name": "Adrien",
    "email": "test@test.com",
    "number": "1"
}
```

## Database

### Env

- `POSTGRES_USER`: Username of the user created
- `POSTGRES_PASSWORD`: Password of the user created
- `POSTGRES_DB`: Name of the database created when startup


## Launching in a kubernetes cluster

### Requirements

- [Kubectl](https://kubernetes.io/docs/tasks/tools/#kubectl) (You must follow the steps depending on your OS)

### Let's gooo !
### Create all namespaces

When cluster creation is done, you can launch these commands to create the namespace for each part of the project : 
```bash
kubectl apply -f ./api/infra/api_namespace.yml
kubectl apply -f ./postgres/infra/postgres_namespace.yml
```

### Deploy apps

The previous part should not be long enough for you to wait so you can deploy each app with the following : 
```bash
kubectl apply -f ./api/infra/
kubectl apply -f ./postgres/infra/
```
These commands apply the **Deploy** files, the **Service** files and any other file that is useful for the project like the **ingress controller** for the webapp or the **secrets** for the api and the db.

PS : You may see that *namespaces are trying to apply again* and you may experiment an error in your console but this is not a problem at all !

#### Check if the pods are ready

The pods are currently pulling the images from the Docker images registry, so it can take some time. You can check the status of the pods with these commands :

- API : `kubectl get pods -n ara-back`
- Database : `kubectl get pods -n ara-db`

### Result

You just need to go to `http://<your_public_url_or_ip>` and you should see the app up and running.

PS : If you did not install an ingress controller, you can use port-forward a port of the pod locally like this : 
```bash
kubectl port-forward pod/<pod_name> <local_port>:8080 -n ara-back
```

# Author
By Adrien RAIMBAULT 
# Kubernetes project : ara-kube


## Global overview

The project is composed of two main services :
- **api** : an classic API using Express, a Javascript framework
- **db** : yet another Postgresql database

## API

### Env

- `POSTGRES_USER`: Username of the user in the postgresql database
- `POSTGRES_PASSWORD`: Password of the user in the postgresql database
- `POSTGRES_HOSTNAME`: Hostname of the machine running postgresql
- `POSTGRES_DB`: Name of the database to connect to

PS : These variables are present in the [.env.example](.env.example) file.

### Routes

Here is a list of the accessible routes :
- **/** : Return 200 - OK
-  **/healthz** : Call the database for date and time

## Database

### Env

- `POSTGRES_USER`: Username of the user created
- `POSTGRES_PASSWORD`: Password of the user created
- `POSTGRES_DB`: Name of the database created when startup

PS : These variables are present in the [.env.example](.env.example) file and are used by the [docker-compose.prod.yml](docker-compose.prod.yml).


## Launch the project locally

The project is ready to use locally, you just need [Docker](https://docs.docker.com/get-docker/) installed !
Here are the steps :

1. Clone the project
2. Go to ara-kube folder
3. Rename `.env.example` file in `.env`
4. Launch the project using :

```
docker compose -f docker-compose.prod.yml up
```

Then you can go to **`http://localhost:8080`** to see the result !


## Launching in K3D

### Requirements

- [Docker](https://docs.docker.com/get-docker/)
- [Kubectl](https://kubernetes.io/docs/tasks/tools/#kubectl) (You must follow the steps depending on your OS)
- [K3D](https://k3d.io/v5.5.1/) or `wget -q -O - https://raw.githubusercontent.com/k3d-io/k3d/main/install.sh | bash`

### Let's gooo !
#### Create a cluster with the k3d cli

```bash
k3d cluster create ara -p "8081:80@loadbalancer"
```

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

You just need to go to `http://localhost:8081` and you should see the app up and running.


### Cleanup
 
 When you are done with this, you can simply delete the cluster :
```bash
k3d cluster rm ara
```

# Author
By Adrien RAIMBAULT 
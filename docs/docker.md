# Docker

This file contains details pertaining to everything related to Docker.

# Table of Contents

- [Building, tagging, and Pushing to Docker Hub](#building-tagging-and-pushing-to-docker-hub)
- [Pulling From Docker Hub in the Droplet](#pulling-from-docker-hub-in-the-droplet)
- [Running `docker compose`](#running-docker-compose)

# Building, tagging, and Pushing to Docker Hub

This section contains instructions for running a manual build, tag, and push of the Dockerized API image to Docker Hub.

1. First, build the API image:

```
cd api/
docker build \
    -t jlai241/stacc-api:<VERSION_NUMBER> . \
    --build-arg STACC_API_PORT_NUMBER=<PORT_NUMBER> \
    --platform linux/amd64
```

> **NOTE:** MAKE SURE TO INCLUDE THE `--platform` FLAG, OTHERWISE IT WON'T RUN ON THE DROPLET BECAUSE IT WAS COMPILED FOR A DIFFERENT ARCHITECTURE.

> **NOTE:** `<VERSION_NUMBER>` should match the version number in `Cargo.toml`, which will/should be incremented after each update.

2. Then, tag the image so that it is reflected in Docker Hub:

```
docker tag jlai241/stacc-api:<VERSION_NUMBER> jlai241/stacc-api:<VERSION_NUMBER>
```

3. Finally, push the image to Docker Hub:

```
docker push jlai241/stacc-api:<VERSION_NUMBER>
```

4. Check if the tag and new image was successfully uploaded to Docker Hub. [Here's a link to the stacc-api private repository in Docker Hub](https://hub.docker.com/repository/docker/jlai241/stacc-api/general)

# Pulling From Docker Hub in the Droplet

Now that the Docker Image has been uploaded to Docker Hub, SSH into the Droplet and pull the new Docker Image:

```
docker image pull jlai241/stacc-api:<VERSION_NUMBER>
```

> **NOTE:** May need to log in before running `docker image pull`:
>
> ```
> docker login
> ```

# Running `docker compose`

Run the following `docker compose` command to load in environment variables and spin up the `api` service:

```
docker compose --env-file .env --env-file .compose-env up -d api
```

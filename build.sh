#!/bin/bash

VERSION_TAG=$1
REPO_NAME="wojtekw92/cert-checker"

echo "Building new image with version $VERSION_TAG"

#TODO: check if everything is commited and puhsed to the repo

docker build . -t REPO_NAME

docker tag ${REPO_NAME}:latest ${REPO_NAME}:$VERSION_TAG

docker push ${REPO_NAME}:latest
docker push ${REPO_NAME}:$VERSION_TAG


# TODO: Get proper tag info
git tag -a $VERSION_TAG -m "????"
git push origin $VERSION_TAG
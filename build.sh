#!/usr/bin/env bash

declare NAME="kottans_pizza_api"

echo "|---------------------------|"
echo "|  BUILD KOTTANS API IMAGE  |"
echo "|---------------------------|"

if [ -z "$1" ]
    then
        echo 'Build version should be supplied.'
        exit 1
fi

echo "Building binary..."
cargo build --release
echo "Build for ${NAME} is done!"

echo "Start building image of ${NAME}..."
cp target/release/${NAME} docker
docker build -t lempiy/${NAME}:$1 ./docker
echo "Successfully built $i!"
echo "Pushing ${NAME}:$1 image to docker-hub..."
docker push lempiy/${NAME}:$1
echo "Image ${NAME}:$1 pushed successfully!"

sed -i -e "s/${NAME}:v.*/${NAME}:${1}/g" ./docker/docker-compose-build.yml
echo "Remove temporal files ..."
rm docker/${NAME}

echo "Build $1 | Local Time: $(date)"

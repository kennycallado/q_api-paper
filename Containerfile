FROM busybox:latest

# --build-arg PACKAGE_NAME=${package_name}
<<<<<<< HEAD
ARG PACKAGE_NAME="q-api-papers"
=======
ARG PACKAGE_NAME="package-name"
>>>>>>> 26669d2dab204045a41f691fbe0bd80de2aa47b7

COPY ./target/x86_64-unknown-linux-musl/release/$PACKAGE_NAME /bin/$PACKAGE_NAME
COPY ./Rocket.toml /root

WORKDIR /root

<<<<<<< HEAD
CMD [ "q-api-papers" ]
=======
CMD [ "package-name" ]
>>>>>>> 26669d2dab204045a41f691fbe0bd80de2aa47b7


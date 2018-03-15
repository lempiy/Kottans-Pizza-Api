FROM ubuntu:16.04
RUN apt-get update \
	&& apt-get install -y \
		openssl \
		git
RUN mkdir /app
COPY kottans_pizza_api* /app/
WORKDIR /app

EXPOSE 3000

ENTRYPOINT /app/kottans_pizza_api

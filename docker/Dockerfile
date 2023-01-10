FROM ubuntu:latest

RUN mkdir /app
RUN mkdir /app/data

COPY ./server /app/server

WORKDIR /app/data

ENTRYPOINT /app/server

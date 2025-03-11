FROM --platform=linux/amd64 node:23 AS build-web

WORKDIR /usr/src/app

COPY web .

RUN npm install

RUN npm run build

FROM --platform=linux/amd64 node:23 AS build-admin

WORKDIR /usr/src/app

COPY admin .

RUN npm install

RUN npm run build

FROM --platform=linux/amd64 rust:1.85 AS build-backend

WORKDIR /usr/src/app

COPY backend .

RUN cargo fetch --locked
RUN cargo build --release --target-dir /target

FROM --platform=linux/amd64 debian:bookworm-slim AS final

RUN apt-get update \
 && apt-get install -y curl \
 && apt-get -y clean \
 && rm -rf /var/lib/apt/lists/*

RUN useradd -m -u 1000 -o -s /bin/bash -d /usr/src/app kp
RUN chown -R 1000 /usr/src/app
USER 1000

COPY --chown=1000 --chmod=774 --from=build-backend /target/release/koudaisai-portal-backend /bin/
COPY --chown=1000 --chmod=774 --from=build-web /usr/src/app/out /var/www/html/web
COPY --chown=1000 --chmod=774 --from=build-admin /usr/src/app/out /var/www/html/admin

EXPOSE 8080

ENTRYPOINT [ "/bin/koudaisai-portal-backend" ]

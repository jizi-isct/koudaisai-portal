FROM debian:bookworm-slim AS build-base
ENV RUST_VERSION="1.84.1"

RUN apt update

RUN apt install -y nodejs curl npm libssl-dev pkg-config libc6

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup install ${RUST_VERSION}


RUN npm install nx -g

WORKDIR /usr/src/app

COPY . .

RUN npm install

FROM build-base AS build-backend

WORKDIR /usr/src/app

RUN nx build --verbose backend

FROM build-base AS build-web

WORKDIR /usr/src/app

RUN nx build --verbose web

FROM debian:bookworm-slim AS final

RUN apt-get update \
 && apt-get install -y build-essential curl \
 && apt-get -y clean \
 && rm -rf /var/lib/apt/lists/*

USER 33

WORKDIR /var/www

COPY --chown=33 --chmod=774 --from=build-backend /usr/src/app/apps/backend/target/release/koudaisai-portal-backend /bin/
COPY --chown=33 --chmod=774 --from=build-web /usr/src/app/apps/web/out /var/www/html/

EXPOSE 8080

ENTRYPOINT [ "/bin/koudaisai-portal-backend" ]

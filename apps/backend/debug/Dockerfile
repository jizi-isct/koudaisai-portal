FROM quay.io/keycloak/keycloak:latest AS builder

# Enable health and metrics support
ENV KC_HEALTH_ENABLED=true
ENV KC_METRICS_ENABLED=true

# Configure a database vendor
ENV KC_DB=postgres

WORKDIR /opt/keycloak
# for demonstration purposes only, please make sure to use proper certificates in production instead
RUN keytool -genkeypair -storepass password -storetype PKCS12 -keyalg RSA -keysize 2048 -dname "CN=server" -alias server -ext "SAN:c=DNS:localhost,IP:127.0.0.1" -keystore conf/server.keystore
RUN /opt/keycloak/bin/kc.sh build

FROM registry.access.redhat.com/ubi9 AS ubi-micro-build
RUN mkdir -p /mnt/rootfs
RUN dnf install --installroot /mnt/rootfs curl --releasever 9 --setopt install_weak_deps=false --nodocs -y && \
    dnf --installroot /mnt/rootfs clean all && \
    rpm --root /mnt/rootfs -e --nodeps setup

FROM quay.io/keycloak/keycloak:latest
COPY --from=ubi-micro-build /mnt/rootfs /
COPY --from=builder /opt/keycloak/ /opt/keycloak/

# change these values to point to a running postgres instance
ENV KC_BOOTSTRAP_ADMIN_USERNAME=admin \
    KC_BOOTSTRAP_ADMIN_PASSWORD=admin \
    KC_DB=postgres \
    KC_DB_URL=jdbc:postgresql://db/koudaisai-portal \
    KC_DB_USERNAME=user \
    KC_DB_PASSWORD=user \
    KC_HEALTH_ENABLED=true \
    KC_METRICS_ENABLED=true \
    KC_HOSTNAME=localhost \
    KC_HOSTNAME_STRICT=false \
    KC_HTTP_ENABLE=true \
    KC_LOGLEVEL=debug \
    KC_HTTP_PORT=8000

HEALTHCHECK --interval=5s --retries=30 CMD curl -f -s http://localhost:8000/health/ready || exit 1
ENTRYPOINT ["/opt/keycloak/bin/kc.sh", "start-dev", "--legacy-observability-interface=true"]
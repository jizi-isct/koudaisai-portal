#!/bin/sh
set -eux

/opt/keycloak/bin/kc.sh start-dev --legacy-observability-interface=true --import-realm
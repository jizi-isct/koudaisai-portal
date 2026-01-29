# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Portal site for 工大祭 (Koudai Festival at Institute of Science Tokyo). Groups submit applications, view resources, and communicate with JIZI (festival organizing committee). Nx monorepo with Next.js frontend and Rust/Axum backend.

## Common Commands

```bash
# Install dependencies (use ci, not install)
npm ci

# Start development (frontend + backend with hot reload)
npx nx dev

# Start Docker services (PostgreSQL, Keycloak)
npx nx docker-up backend

# Build
npx nx build web      # Frontend (Next.js SSG → apps/web/out/)
npx nx build backend  # Backend (Rust release)

# Test
npx nx test backend   # cargo test

# Lint
npx nx lint web       # ESLint
npx nx lint backend

# Database migrations (in apps/backend/)
sea-orm-cli migrate up
sea-orm-cli generate entity  # Generate SeaORM entities from DB schema
```

## Architecture

### Monorepo Structure
- `apps/web/` - Next.js 15 frontend (TypeScript, Ant Design, SSG)
- `apps/backend/` - Rust/Axum backend (SeaORM, PostgreSQL)
- `docs/` - OpenAPI specs and documentation

### Backend Layers (`apps/backend/src/`)
1. `routes/` - HTTP endpoints
2. `middlewares.rs` - Auth, logging
3. `entities/` - Business logic
4. `sea_orm_entities/` - DB models (auto-generated)
5. `service/` - External integrations (Discord, S3)
6. `util/` - JWT, OIDC, hashing

### Authentication
- **JIZI (admins)**: Keycloak OIDC
- **Groups**: Custom JWT with activation on first login

### External Services
- PostgreSQL (data), S3/Wasabi (files), Keycloak (auth)
- Discord (notifications), SendGrid/SES (email)
- External API: api2025.jizi.jp (project info sync)

## Domain Terms

- **JIZI**: Festival organizing committee (admins)
- **Group**: Participating unit (Project or Press)
- **Project types**: Booth (模擬店 M-xxx), Stage (S-xxx), General (一般 I-xxx), Labo (研究室 L-xxx)
- **Press**: Media/coverage groups (P-xxx)

## Development Setup

1. Install: Rust, Node.js, Docker, Docker Compose, cargo-watch
2. `npm ci`
3. Generate JWT keys: `cd apps/backend/debug && ./init-keys.sh`
4. Copy config: `cp apps/backend/debug/default-config.toml <OS-config-path>/`
   - macOS: `~/Library/Application Support/rs.koudaisai-portal/`
   - Linux: `~/.config/koudaisai-portal/`
5. `npx nx docker-up backend`
6. `npx nx dev` → Frontend: localhost:3000, Backend: localhost:8000

## Branch Naming

```
<prefix>/#<issue>-<short-title>
```
Prefixes: feature, fix, hotfix, refactor, chore, test

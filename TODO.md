# TODO

- Replace manual Google JWK fetch/validation with an OpenID Connect client (discover endpoints + JWKS).
- Issue first-party access/refresh tokens after initial Google auth so we can refresh sessions ourselves (access 15 min, refresh 30 days).
- Review SQLite tables and queries for correctness and indexing.
- Backend: move models to an `entities` folder and detach them from `infra`.
- Review the Google auth flow in the frontend.
- Move PWA assets/config to the nginx frontend container.
- Configure Fly.io SQLite storage (volume mount, single region, DATABASE_PATH).
- Update the frontend page title.
- Configure GitHub Actions workflow for frontend deploy.

# TODO

- Replace manual Google JWK fetch/validation with an OpenID Connect client (discover endpoints + JWKS).
- Issue first-party access/refresh tokens after initial Google auth so we can refresh sessions ourselves (access 15 min, refresh 30 days).
- Review SQLite tables and queries for correctness and indexing.
- Backend: move models to an `entities` folder and detach them from `infra`.

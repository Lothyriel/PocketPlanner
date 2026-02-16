# TODO

- Replace manual Google JWK fetch/validation with openidconnect
- Issue first-party access/refresh tokens after initial Google auth so we can refresh sessions ourselves (access 15 min, refresh 30 days).
- Review SQLite tables and queries for correctness and indexing.
- Backend: move models to an `entities` folder and detach them from `infra`.
- Review the Google auth flow in the frontend.
- Move PWA assets/config to the nginx frontend container. (and repo folder)
- Update the frontend page title.

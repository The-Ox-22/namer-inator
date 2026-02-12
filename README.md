# The Namer-inator

> *"Behold! The Namer-inator! With this, I will assign a name to every evil scheme across the entire TRI-STATE AREA!"*

An API-inator that serves up random inator names from Dr. Doofenshmirtz's extensive catalog of evil inventions.

## The Run-inator

```bash
cargo run
```

The server-inator starts at `http://0.0.0.0:8080`. Set the `HOST` environment variable to change the bind-inator address.

## The Endpoint-inators

### GET `/random-inator`

Returns a random inator from across all seasons and beyond.

```json
{"inator": "Deflate-inator / Deflate-inator Ray"}
```

### GET `/random-inator/pure`

Returns only the purest of inators -- those that end with the sacred "-inator" suffix, as Doofenshmirtz intended.

```json
{"inator": "Destruct-inator"}
```

### GET `/random-inator/{season}`

Returns a random inator from a specific season-inator.

Valid options: `season_1`, `season_2`, `season_3`, `season_4`, `season_5`, `outside_main_series`, `pure`

## The Format-inator

All endpoints accept optional query parameters for formatting the inator names.

### `case`

Controls how spaces and hyphens in the name are replaced.

| Value | Example-inator |
|---|---|
| *(none)* | `Space-Laser-inator` |
| `snake` | `Space_Laser_inator` |
| `camel` | `SpaceLaserInator` |
| `kebab` | `Space-Laser-inator` |
| `no_spaces` | `SpaceLaserinator` |
| `lower` | `space-laser-inator` |
| `upper` | `SPACE-LASER-INATOR` |

### `strip_special`

Set to `true` to remove special characters (parentheses, question marks, slashes, etc.) while preserving letters, numbers, spaces, and hyphens.

### The Combo-inator

Both parameters can be combined for maximum evil:

```
GET /random-inator?case=snake&strip_special=true
```

## The Deploy-inator

### Docker-inator

```bash
docker build -t namer-inator .
docker run -p 8080:8080 namer-inator
```

### Helm-inator

```bash
helm install namer-inator ./chart
```

## The Stack-inator

- **Rust** - The language-inator
- **Actix-web** - The framework-inator
- **Serde** - The serialize-inator

## The License-inator

See [LICENSE](LICENSE) for details. *A platypus? ...Perry the Platypus?!*

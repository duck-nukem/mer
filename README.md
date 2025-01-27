# mér

Aggregated energy usage stats

## Setup

Current architecture consists of:
* main app
* postgres database
* in-memory caching

### Prerequisites

Something that runs containers, e.g. docker, podman, orbstack, etc

#### Up & Running

- `docker compose up -d` to bring up the dependencies
- `./dev.sh` runs the above + starts the project with watch mode
- `./sql.sh` drops you in the postgres console


## Good to know

### Security

#### Session

Sessions are entirely stored in cookies in the form of JWT tokens. JWT is an overkill probably, but it was already
implemented in the framework, so I left it at that.

#### CSP

Mandatory requirement to use a strict CSP on all pages. The nonce is already generated and is exposed & accessible
through the `Extension(nonce): Extension<CspNonce>` extractor

### Session

#### Flash messages

GET -> user action -> POST -> response; in this cycle we might want to show a message to the user.
Current solution is to simply use query params to pass along messages.

```rust
async fn my_view(
  ViewEnginer(v): ViewEngine<TeraView>,
  Query(query_params): Query<PageState>,
) -> Result<impl IntoResponse> {
  views::view_fn(&v, &query_params)
}
```

Alternatively, you can extract ad-hoc params via `Query(params): Query<HashMap<String, String>>`.

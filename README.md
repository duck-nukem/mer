# mér

Aggregated energy usage stats

## Setup

### Prerequisites

Something that runs containers, e.g. docker, podman, orbstack, etc

#### Up & Running

- `docker compose up -d` to bring up the dependencies
- ./dev.sh runs the above + starts the project with watch mode


## Good to know

### Security

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

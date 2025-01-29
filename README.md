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

#### Auth Session

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
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct PageState {
    pub status: Option<PageStatus>,
    pub message: Option<String>,
}

async fn my_view(
  ViewEngine(v): ViewEngine<TeraView>,
  Query(query_params): Query<PageState>,
) -> Result<impl IntoResponse> {
  views::view_fn(&v, &query_params)
}
```

Alternatively, you can extract ad-hoc params via `Query(params): Query<HashMap<String, String>>`.

#### Routers & middlewares

In `src/routes.rs` there are two top-level functions where you can define the routes from your apps,
and the middlewares you want to apply to all of them.

Be mindful which routes you include here, for example if one of the middlewares redirects unauthenticated
users to login, you shouldn't add the authentication related app routes to this config!

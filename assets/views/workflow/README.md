# Microfrontend architecture

Most of the application doesn't require reactivity, but when it does, we can build individual, independent, small frontends.

Common functionality, and ui components needs to be figured out later on

## Build

To see this in action, run `npm run build:watch` and all the relevant files will be moved to the correct place; static files will
be served from the rust app. Routing to `/dist/index.html` should be set up in the rust module's router

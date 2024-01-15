## Cleanup

- add utility functions for handling errors in view (like badRequest(message), internalError(message), etc)
- wrap fetching summaries in an abstraction
- failing to fetch a summary should not panic (use Result)

## View

## Summary Refinement

- Can we get rid of any prefaces like "this diff..."
- It seems confused about diffs to documentation

## Infrastructure

- Can we make processing more async?
- Can we batch summary requests to save money?

## Auth

- Add CSRF safety like in https://github.com/maxcountryman/axum-login/blob/main/examples/oauth2/src/web/app.rs

## Personalization

### Subscribe to repos
- Add page to subscribe to repos
- Can unsubscribe from repos
- Add a webhook handler for removing repo when the user loses access.
- Only display items in feed from subscribed repositories

### Collect info on relevance
- What files does the user often touch
- is the commiting user someone with overlapping commit histories?
- What if the commit is from someone in a shared group (maybe codeowners?)
- Is this change really big?

### Vector DB / Embeddings
- Can we keep track of user interests (like from above) and then use a search to get relevant pushes?

### Can we explain why the change is relevant to the user?
- Feed the above info plus the existing commit summary into the summary model?

## Multitenant

- Do not allow users to see commits from repos they dont have permissions for.

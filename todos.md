## Cleanup

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

- Only show info for repositories you have access to
- Hard coded rules about who might be interested in what
- Discovery work with openai tooling

## Multitenant

- Auth via github
- Do not allow users to see commits from repos they dont have permissions for.

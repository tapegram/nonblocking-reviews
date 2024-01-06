## Cleanup and deploy

- deploy application
- wrap fetching summaries in an abstraction
- failing to fetch a summary should not panic (use Result)

## Summary Refinement

- Include commit messages in prompt
- Tweak prompt for shorter responses

## Infrastructure

- Can we make processing more async?
- Can we batch summary requests to save money?

## Personalization

- Auth
- Hard coded rules about who might be interested in what
- Discovery work with openai tooling

## Multitenant

- Auth via github
- Do not allow users to see commits from repos they dont have permissions for.

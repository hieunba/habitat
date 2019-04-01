#!/bin/bash

msg="Hello $EXPEDITOR_AUTHOR! Thanks for the pull request! Here is what will happen next:\n 1. Your PR will be reviewed by the maintainers\n 2. If everything looks good, one of them will approve it, and your PR will be merged.\n\nThank you for contributing!"

curl -X POST \
  -H "Content-Type: application/vnd.github.3.raw+json" \
  -H "Authorization: token $GITHUB_TOKEN" \
  -d "{\"body\":\"$msg\"}" \
  "https://api.github.com/repos/$EXPEDITOR_REPO/issues/$EXPEDITOR_NUMBER/comments"

# PR Has Issues GitHub Action

This action checks for issue references in a PR title and body.

## Inputs

## `prefixes`

**Required** A comma separated list of issue key prefixes.

Example value: `ISSUE-` or `PLAT-,ENG-,IT-`

## Outputs

n/a

## Example usage

```yaml
name: PR

on:
  pull_request:
    types: [opened, reopened, edited, ready_for_review]

concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true

jobs:
  check:
    name: Check
    runs-on: ubuntu-latest
    steps:
      - uses: ngerakines/pr-has-issues-action@v5
        with:
          prefixes: PLAT-,ENG-,IT-
```

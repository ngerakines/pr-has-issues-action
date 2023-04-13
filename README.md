# PR Has Issues GitHub Action

This action checks for issue references in a PR title and body.

## Inputs

## `key_prefixes`

**Required** A comma separated list of key prefixes.

Example value: `ISSUE-` or `PLAT-,ENG-,IT-`

## Outputs

n/a

## Example usage

```yaml
uses: ngerakines/pr-has-issues-action
with:
  key_prefixes: PLAT-,ENG-,IT-
```

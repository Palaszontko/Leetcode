# My Leetcode solutions repository

This repository is dedicated to my solutions for the **NeetCode 150** and also includes solutions to other **LeetCode** challenges.

NeetCode 150 is a curated list of 150 LeetCode problems designed to cover all possible patterns & topics in data structures.

Creator's [original repository](https://github.com/neetcode-gh/leetcode) and [website](https://neetcode.io/).

## Structure

```
go/<id>.<slug>/solution.go
```

Example: `go/0739.daily-temperatures/solution.go`.

## Workflow with [leetgo](https://github.com/j178/leetgo)

```sh
leetgo pick <id> -l go        # generates go/<id>.<slug>/
leetgo test last -L           # run local tests against testcases.txt
leetgo submit last            # submit to LeetCode
```

Auth credentials live in `.env` (gitignored). Project config in `leetgo.yaml`.

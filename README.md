# doe

[![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](https://choosealicense.com/licenses/mit/)

Minimal and simplistic environment (`.env`) file manager CLI tool. doe helps you
create and update env files as well as change environments quickly and easily.

The primary purpose is making it easier for developers to work with `.env` files
locally, however it has benefits in CI/CD automation as well.

## Installation

At the moment, doe is still under development, but the goal is to make it
available as a pre-built binary for Windows, Linux and Mac OS.
    
## Usage/Examples

*Note: This is a very early draft of what the interface will be like and is
subject to change.*

```
$ doe list                             # list all envs in current folder/project
* local
prod
test

$ doe local                            # switch current env to .env.local
* local

$ doe prod                             # switch current env to .env.prod
* prod

# doe test                             # can't change to a non-existent env
! .env.test not found

$ doe new test prod                    # create and switch to .env.test based on prod
* test

$ doe DB_NAME=my_test_database         # set an env var in the current env
* test
DB_NAME=my_test_database

$ doe DB_USER                          # display value of an env var in the current env
* prod
DB_USER=aalaap 
```

## Roadmap

- [ ] Initial release
- [ ] Auto-generate `.env` from code
- [ ] `.env.dist` support
- [ ] Undo
- [ ] Cloud backup/sync
- [ ] Sharing with team 

## Authors

- Aalaap Ghag ([@aalaap](https://github.com/aalaap))

## License

[MIT](https://choosealicense.com/licenses/mit/)

## Repos


### Terminal ui app to manage multiple git repositories in a dev dir.

App that comes in handy when you have **multiple repos to work in and they depend on each other**. In workflows like this, when working on smaller tasks checking all the required repos` status and **switching branches for different tasks** can be quite boring. Well, not anymore. Put this app on the sys path, set up the devdir env var and run this anytime anywhere.

![](demo_render/repos_demo.gif)


## Usage

- Set the `env var`: `DEVDIR`. Put this on the system path. Without the env var the current dir is used.
- or, use a dir as the first argument.

### navigation:

- quit: **`q`**
- down: **`j`**
- up: **`k`**
- left: **`h`**
- right: **`l`**
- checkout branch: `enter` on highlighted branch
- sort: **`s`** - sort by `branch, status or current branch`


### colour codes:

- `green`: current branch is `master`, and the `status is clean` other than untracked files.
- `cyan`: the `status is clean`, a branch other then master is checked out.
- `yellow`: `master` is checked out, but the status is not clean.
- `red`: the status is not clean and a branch other than master is checked out.
- `gray`: *in branches* - existing branch
- `green`: *in branches* - current branch


### todo:  
    - 'd' for delete branch
    - 'c' to create branch
    - reselect cell after sort
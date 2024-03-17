# @seboran/ni

This project is a cli to handle multiple package managers. One could say it's a package managers' manager. Works* with bun, yarn, npm and pnpm!

This project is like <https://github.com/antfu/ni> but in rust, because why not?

*At the moment, it only works with bun as the global package manager...

## Install

```sh
bun add -g @seboran/ni
```

## Features

### Install package

```sh
bun ni
# npm i
# yarn install
# pnpm i
# bun i
```

```sh
bun ni -D vite
# npm i -D vite
# pnpm add -D vite
# bun add -D vite
```

```sh
bun ni -g nodemon
# npm i -g nodemon
# yarn global add nodemon
# pnpm add -g nodemon
# bun add -g nodemon
```

### Alias

```sh
bun na
# npm
# yarn
# pnpm
# bun
```

### Run script

```sh
bun nr my_script
# npm run my_script
# yarn my_script
# pnpm my_script
# bun my_script
```

### Remove package

```sh
bun nrm vite
# npm rm vite
# yarn remove vite
# pnpm rm vite
# bun rm vite
```

```sh
bun nrm -g vite
# npm rm -g vite
# yarn global remove vite
# pnpm rm -g vite
# bun rm -g vite
```

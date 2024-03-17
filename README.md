# @seboran/ni

This project is a cli to handle multiple package managers. One could say it's a package managers' manager. Works with bun, yarn, npm and pnpm!

This project is like <https://github.com/antfu/ni> but in rust, because why not?

## Install

```sh
bun add -g @seboran/ni
```

## Features

### Install package

```sh
ni
# npm i
# yarn install
# pnpm i
# bun i
```

```sh
ni -D vite
# npm i -D vite
# pnpm add -D vite
# bun add -D vite
```

```sh
ni -g nodemon
# npm i -g nodemon
# yarn global add nodemon
# pnpm add -g nodemon
# bun add -g nodemon
```

### Alias

```sh
na
# npm
# yarn
# pnpm
# bun
```

### Run script

```sh
nr my_script
# npm run my_script
# yarn my_script
# pnpm my_script
# bun my_script
```

### Remove package

```sh
nrm vite
# npm rm vite
# yarn remove vite
# pnpm rm vite
# bun rm vite
```

```sh
nrm -g vite
# npm rm -g vite
# yarn global remove vite
# pnpm rm -g vite
# bun rm -g vite
```

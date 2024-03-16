# @seboran/ni

This project is like <https://github.com/antfu/ni> but in rust, because why not?

## Install

```sh
npm i -g @seboran/ni
```

## Features

### Install package

Examples

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

### Run script

```sh
nr my_script
# npm run my_script
# yarn my_script
# pnpm my_script
# bun my_script
```

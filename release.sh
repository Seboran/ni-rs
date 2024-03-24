#!/bin/bash
cd npm
pnpm -r publish --access public --tag
pnpm publish --access public --tag
#!/usr/bin/env just --justfile

css:
    npx @tailwindcss/cli -i ./res/css/tailwind_input.css -o ./res/css/tailwind_output.min.css --watch --minify
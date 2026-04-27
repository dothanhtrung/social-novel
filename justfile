#!/usr/bin/env just --justfile

VERSION := `cd social-novel && cargo pkgid | sed 's/.*#//'`

css:
    npx @tailwindcss/cli -i ./res/css/tailwind_input.css -o ./res/css/tailwind_output.min.css --watch --minify

windows:
    cargo build --target=x86_64-pc-windows-gnu --release
    rm -rf output/windows
    mkdir -p output/windows/social-novel
    cp target/x86_64-pc-windows-gnu/release/social-novel.exe output/windows/social-novel/
    cp -r res output/windows/social-novel/
    cp social-novel-config-sample.ron output/windows/social-novel/social-novel.ron
    cd output/windows && zip -r social-novel_{{VERSION}}.windows.zip social-novel && mv social-novel_{{VERSION}}.windows.zip ..

linux:
    cargo build --target=x86_64-unknown-linux-musl --release
    rm -rf output/linux
    mkdir -p output/linux/social-novel
    cp target/x86_64-unknown-linux-musl/release/social-novel output/linux/social-novel/
    cp -r res output/linux/social-novel/
    cp social-novel-config-sample.ron output/linux/social-novel/social-novel.ron
    cd output/linux && tar cJvf social-novel_{{VERSION}}.linux.tar.xz social-novel && mv social-novel_{{VERSION}}.linux.tar.xz ..

release: windows linux

watch:
    bacon run-long -- -- -c ./social-novel.ron

clean:
    cargo sweep -t 30

social-novel
============

Make social story for your favorite characters.

> IN PROGRESS  
> Backend is human slop. Frontend is mixed slop.

How to run
----------

### Database

* Start PostgreSQL (ignore if you already have the database run elsewhere):

```shell
cd db
docker compose up -d
```

### Application

Edit [social-novel.ron](./social-novel.ron) to match your database address.

Run applicaiton:
```shell
./social-novel -c ./social-novel.ron
```

How to build
------------

```shell
cargo build --release
```

---------

<div align="center">

![git_social-novel](https://count.getloli.com/@git_social-novel?name=git_social-novel&theme=random&padding=10&offset=0&align=top&scale=1&pixelated=1&darkmode=auto)

</div>
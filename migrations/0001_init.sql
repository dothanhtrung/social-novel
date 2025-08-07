create table if not exists _sqlx_migrations
(
    version        BIGINT
        primary key,
    description    TEXT                                not null,
    installed_on   TIMESTAMP default CURRENT_TIMESTAMP not null,
    success        BOOLEAN                             not null,
    checksum       BLOB                                not null,
    execution_time BIGINT                              not null
);

create table if not exists character
(
    name     TEXT    not null,
    username TEXT    not null
        constraint character_pk_2
            unique,
    id       integer not null
        constraint character_pk
            primary key autoincrement
);

create table if not exists post
(
    id         integer                                 not null
        constraint post_pk
            primary key autoincrement,
    content    TEXT                                    not null,
    liked      integer default 0                       not null,
    created_at INTEGER default (strftime('%s', 'now')) not null,
    updated_at INTEGER default (strftime('%s', 'now')) not null,
    parent     integer
        constraint post_post_id_fk
            references post
            on delete cascade,
    author     integer default 1                       not null
        constraint post_character_id_fk
            references character
            on delete cascade,
    haha       integer default 0                       not null,
    loved      integer default 0                       not null,
    surprised  integer default 0                       not null,
    sad        integer default 0                       not null
);

create table if not exists media
(
    id        integer not null
        constraint media_pk
            primary key autoincrement,
    url       TEXT    not null
        constraint media_pk_2
            unique,
    file_type integer not null,
    post      integer not null
        constraint media_post_id_fk
            references post
            on delete cascade,
    blake3    text    not null
        constraint media_pk_3
            unique
);



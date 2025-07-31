create table character
(
    name     TEXT    not null,
    username TEXT    not null
        constraint character_pk_2
            unique,
    id       integer not null
        constraint character_pk
            primary key autoincrement
);

create table post
(
    id         integer                           not null
        constraint post_pk
            primary key autoincrement,
    content    TEXT                              not null,
    liked      integer default 0                 not null,
    created_at INTEGER default current_timestamp not null,
    updated_at INTEGER default current_timestamp not null,
    parent     integer
        constraint post_post_id_fk
            references post
            on delete cascade,
    author     integer default 1                 not null
        constraint post_character_id_fk
            references character
            on delete cascade,
    haha       integer default 0                 not null,
    loved      integer default 0                 not null,
    surprised  integer default 0                 not null
);

create table media
(
    id   integer not null
        constraint media_pk
            primary key autoincrement,
    url  TEXT    not null,
    type integer,
    post integer not null
        constraint media_post_id_fk
            references post
            on delete cascade
);



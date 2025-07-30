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
    id          integer                        not null
        constraint post_pk
            primary key autoincrement,
    content     TEXT,
    liked       integer,
    created_at  TEXT default current_timestamp not null,
    modified_at TEXT default current_timestamp not null,
    parent      integer
        constraint post_post_id_fk
            references post
            on delete cascade,
    author      id
        constraint post_character_id_fk
            references character
            on delete cascade,
    haha        integer,
    loved       integer,
    surprised   integer
);

create table if not exists media
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



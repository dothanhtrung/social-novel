create table if not exists character
(
    name     text    not null,
    username varchar not null
        constraint character_pk_2
            unique,
    id       bigint generated always as identity
        constraint character_pk
            primary key
);

create table if not exists post
(
    id         bigint generated always as identity
        constraint post_pk
            primary key,
    content    text                     default ''::text not null,
    liked      integer                  default 0        not null,
    created_at timestamp with time zone default now()    not null,
    updated_at timestamp with time zone default now()    not null,
    parent     bigint
        constraint post_post_id_fk
            references post,
    author     bigint                                    not null
        constraint post_character_id_fk
            references character,
    haha       integer                  default 0        not null,
    loved      integer                  default 0        not null,
    surprised  integer                  default 0        not null,
    sad        integer                  default 0        not null,
    feeling    text                     default ''::text not null,
    is_with    text                     default ''::text not null
);

create table if not exists media
(
    id        bigint generated always as identity
        constraint media_pk
            primary key,
    url       text                      not null
        constraint media_pk_2
            unique,
    file_type smallint default 0        not null,
    post      bigint                    not null
        constraint media_post_id_fk
            references post,
    blake3    text     default ''::text not null
);

comment on column media.file_type is '0=NA, Image, Video, Audio';



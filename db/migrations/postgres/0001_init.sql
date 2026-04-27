create sequence group_id_seq
    as integer;

create sequence group_character_id_seq;

create table if not exists character
(
    name        text                      not null,
    username    varchar                   not null
        constraint character_pk_2
            unique,
    id          bigint generated always as identity
        constraint character_pk
            primary key,
    description text  default ''::text    not null,
    bio         jsonb default '{}'::jsonb not null
);

create table if not exists relation
(
    id             bigserial
        constraint relation_pk
            primary key,
    from_character bigint                not null
        constraint relation_character_id_fk
            references character
            on update cascade on delete cascade,
    to_character   bigint                not null
        constraint relation_character_id_fk_2
            references character
            on update cascade on delete cascade,
    relation       text default ''::text not null
);

create table if not exists groups
(
    id          bigint default nextval('group_id_seq'::regclass) not null
        constraint group_pk
            primary key,
    name        text                                             not null,
    description text   default ''::text                          not null
);

alter sequence group_id_seq owned by groups.id;

create table if not exists group_member
(
    id        bigint default nextval('group_character_id_seq'::regclass) not null
        constraint group_member_pk
            primary key,
    character bigint                                                     not null
        constraint group_member_character_id_fk
            references character,
    "group"   bigint                                                     not null
        constraint group_member_group_id_fk
            references groups,
    role      integer                                                    not null
);

alter sequence group_character_id_seq owned by group_member.id;

create table if not exists ads
(
    id          bigserial
        constraint ads_pk
            primary key,
    description text    default ''::text not null,
    active      boolean default true     not null
);

create table if not exists chat_room
(
    id   bigserial
        constraint chat_room_pk
            primary key,
    name text default ''::text not null
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
    is_with    text                     default ''::text not null,
    "group"    bigint
        constraint post_group_id_fk
            references groups
            on update cascade on delete cascade,
    room       bigint
        constraint post_chat_room_id_fk
            references chat_room
            on update cascade on delete cascade
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

create table if not exists chat_room_member
(
    id        bigserial
        constraint chat_room_member_pk
            primary key,
    room      bigint
        constraint chat_room_member_chat_room_id_fk
            references chat_room
            on update cascade on delete cascade,
    character bigint            not null
        constraint chat_room_member_character_id_fk
            references character
            on update cascade on delete cascade,
    role      integer default 0 not null
);



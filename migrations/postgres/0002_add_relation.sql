alter table public.character
    add if not exists description text default '' not null;

create table if not exists public.relation
(
    id             bigserial
        constraint relation_pk
            primary key,
    from_character bigint          not null
        constraint relation_character_id_fk
            references public.character
            on update cascade on delete cascade,
    to_character   bigint          not null
        constraint relation_character_id_fk_2
            references public.character
            on update cascade on delete cascade,
    relation       text default '' not null
);

create table if not exists public."group"
(
    id          bigserial
        constraint group_pk
            primary key,
    name        text            not null,
    description text default '' not null
);

alter table public.post
    add if not exists "group" bigint
        constraint post_group_id_fk
            references public."group"
            on update cascade on delete cascade;

create table if not exists public.group_member
(
    id        bigserial
        constraint group_member_pk
            primary key,
    character bigint  not null
        constraint group_member_character_id_fk
            references public.character
            on update cascade on delete cascade,
    "group"   bigint  not null
        constraint group_member_group_id_fk
            references public."group"
            on update cascade on delete cascade,
    role      integer not null
);

create table if not exists public.ads
(
    id          bigserial
        constraint ads_pk
            primary key,
    description text default '' not null
);


create table if not exists public.chat_room
(
    id   bigserial
        constraint chat_room_pk
            primary key,
    name TEXT default '' not null
);

create table if not exists public.chat_room_member
(
    id        bigserial
        constraint chat_room_member_pk
            primary key,
    room      bigint
        constraint chat_room_member_chat_room_id_fk
            references public.chat_room
            on update cascade on delete cascade,
    character bigint            not null
        constraint chat_room_member_character_id_fk
            references public.character
            on update cascade on delete cascade,
    role      integer default 0 not null
);



alter table public.post
    add room bigint
        constraint post_chat_room_id_fk
            references public.chat_room
            on update cascade on delete cascade;


alter table public.character
    add if not exists bio jsonb default '{}' not null;

alter table public.ads
    add if not exists active bool default true not null;

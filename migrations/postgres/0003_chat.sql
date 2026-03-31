create table if not exists public.chat_room
(
    id   bigserial
        constraint chat_room_pk
            primary key,
    name TEXT default '' not null
);

create table if not exists public.chat_message
(
    id         bigserial
        constraint chat_message_pk
            primary key,
    sender     bigint                    not null
        constraint chat_message_character_id_fk
            references public.character
            on update cascade on delete cascade,
    room       bigint                    not null
        constraint chat_message_chat_room_id_fk
            references public.chat_room
            on update cascade on delete cascade,
    content    text        default ''    not null,
    media      text,
    created_at timestamptz default now() not null
);

alter table public.character
    add if not exists bio jsonb default '{}' not null;

alter table public.ads
    add if not exists active bool default true not null;
create table if not exists friend
(
    id         bigserial
        constraint friend_pk
            primary key,
    character  bigint                                 not null
        constraint friend_character_id_fk
            references character
            on update cascade on delete cascade,
    friend     bigint                                 not null
        constraint friend_character_id_fk_2
            references character
            on update cascade on delete cascade,
    created_at timestamp with time zone default now() not null,
    constraint friend_not_self check (character <> friend)
);

-- One undirected pair, stored as a single row in either order.
create unique index if not exists friend_pair_uidx
    on friend (least(character, friend), greatest(character, friend));

create index if not exists friend_character_idx on friend (character);
create index if not exists friend_friend_idx on friend (friend);

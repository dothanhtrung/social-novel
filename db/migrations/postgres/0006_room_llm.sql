alter table chat_room
    add column if not exists llm_enabled boolean default false not null,
    add column if not exists llm_recent integer default 10 not null;

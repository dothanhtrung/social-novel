alter table character
    add column if not exists back_story text default ''::text not null;

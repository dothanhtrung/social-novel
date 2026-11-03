-- One media file can belong to many posts.
create table if not exists post_media
(
    post_id  bigint not null
        constraint post_media_post_id_fk
            references post
            on update cascade on delete cascade,
    media_id bigint not null
        constraint post_media_media_id_fk
            references media
            on update cascade on delete cascade,
    constraint post_media_pk
        primary key (post_id, media_id)
);

create index if not exists post_media_media_id_idx on post_media (media_id);

-- Copy the old one-post link, then drop it. Guarded so the migration can run twice.
do $$
begin
    if exists (
        select 1
        from information_schema.columns
        where table_schema = 'public'
          and table_name = 'media'
          and column_name = 'post'
    ) then
        execute $copy$
            insert into post_media (post_id, media_id)
            select post, id from media
            on conflict do nothing
        $copy$;
        alter table media drop constraint if exists media_post_id_fk;
        alter table media drop column post;
    end if;
end $$;

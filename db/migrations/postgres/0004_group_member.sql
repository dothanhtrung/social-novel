-- Drop duplicate memberships so the unique pair index can be created.
delete from group_member a
using group_member b
where a.id > b.id
  and a.character = b.character
  and a."group" = b."group";

create unique index if not exists group_member_pair_uidx
    on group_member (character, "group");

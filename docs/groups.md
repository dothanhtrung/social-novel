# Groups

## Group page

The group page is the home feed template (`res/html/index.html`) rendered at `GET /g/{id}`.

* The server puts the id on `<body data-group-id>`. `0` means the home feed.
* Home sidebar rows come from `GET /api/group?member={selected character}` and link to `/g/{id}`. The row whose id matches `data-group-id` is marked selected (`bg-[#E7F3FF]`, blue bold text, `aria-current="page"`). On a group page that list is replaced by the group info and `GET /api/group/members?group={id}`.
* `/group` renders `res/html/groups.html`. It loads every group from `GET /api/group` (optional `search`) and marks Join or Leave from the same member filter.
* A Facebook-style group header sits above the composer: cover image (group avatar with a blue wash), rounded avatar, name, "Public group", description, and a Discussion tab. The composer placeholder and a "Posting in {name}" line show the selected group.
* Edit opens the same modal as Add, prefilled with the current name, description, and avatar. Submit is `POST /api/group` with the existing id. An empty avatar field is omitted so the stored image is kept. Avatar URLs include a cache-busting query after a save. Creating a group returns its `id`; the selected character is then joined so it shows under Your groups.
* Join is `POST /api/group/join?character={id}&group={id}`. Leave is `GET /api/group/leave?character={id}&group={id}`. A unique index on `(character, "group")` makes a second join a no-op. The Join button is on each `/group` card and on the group page (header and sidebar). It is hidden until a character is selected.
* The feed query is `GET /api/post?group={id}&roots=true` plus the usual `page` and `count`. `roots=true` keeps comments out of the post list so they only appear under their parent.

## Belonging to a group

* `post.group` is the group id, or null when the post is not in a group.
* The home composer sends `group=0` unless a group page is open, in which case it sends the selected id.
* Each comment form carries `data-group-id` from the post it is attached to. A reply box copies that id from the post's form. The comment request sends that id.
* On insert, if `group` is 0 and `parent` is set, the API copies `group` from the nearest ancestor that has one. Edits do not inherit, so the edit form can keep or clear the stored group.
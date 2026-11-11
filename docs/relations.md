# Relations

The admin page (`res/html/admin.html`, `GET /admin`) has a Relations tab next to Users and Sponsored.

* `relation.from_character` is `relation.relation` of `relation.to_character`. Names and usernames on the list come from a join to `character`.
* List: `GET /api/admin/relation`. Optional `character` keeps rows where that id is either side. `GET /api/admin/relation/{id}` returns one row in the same shape (`relations`, `err`).
* Save is `POST /api/admin/relation` with JSON `{id, from_character, to_character, relation}`. `id` of `0` inserts; a positive id updates. Delete is `GET /api/admin/relation/delete/{id}`.
* The form's character dropdowns and the list filter share `GET /api/admin/user`. Each dropdown has its own search box. When a character is filtered, New pre-fills From with that character.
* Each user card has a Relations button that opens this tab filtered to that character, with From already set for a new relation.
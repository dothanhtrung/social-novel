# Characters

## Back story

Admin-only text on `character.back_story` (`text`, default empty). It is meant for later LLM prompts and is not shown on the public profile or group/home UI.

* Column: `db/migrations/postgres/0005_character_back_story.sql`.
* Admin Users form (`res/html/admin.html`) has a Back story textarea. Save is `POST /api/admin/user` (multipart) with `back_story`. List/get is `GET /api/admin/user` and includes the field when it is not empty.
* `GET /api/character` omits `back_story`. Editing a profile through `POST /api/character` keeps the stored back story unchanged.
* A messenger LLM reply includes this text as `{sender_back_story}` or `{receiver_back_story}` inside `config.llm.chat_setup_prompt`.

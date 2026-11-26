# Settings

The admin page (`res/html/admin.html`, `GET /admin`) has a Settings tab next to Users, Sponsored, and Relations.

* Live values come from `ConfigData` (`crates/config`). Save writes the same struct to the RON file at `config_path` (overwrite) and replaces the in-memory config.
* After a successful save the HTTP server restarts so bind address, database pool, `/data` files, and basic-auth enable/disable pick up the new values. Request-time fields (`data_dir` for uploads, `api.per_page`, auth credentials) also update in the `RwLock` immediately.
* Read: `GET /api/admin/config` returns `{config, path, err}`. `config` is the `Config` struct. `path` is the file this process loaded and will overwrite on save. The Settings tab shows that path. A second RON file in the repo (for example `social-novel.ron` when the process was started with `social-novel.test.ron`) is not the file Save writes.
* `api.per_page` is how many root posts the home feed and a group feed request per page. Those pages omit `count`, so this value is the limit. Profile (`count=100`) and chat (`count=50`) send their own count and are not capped by it.
* Save: `POST /api/admin/config` with JSON matching `Config`. Empty data directory, empty listen address, port `0` or above `65535`, or `api.per_page` of `0` are rejected. The items-per-page field must be an integer of at least 1.
* The form shows the postgres `db_path` as Database URL. SQLite settings from the loaded config are sent back unchanged.
* LLM fields are `config.llm.url` (llama.cpp server base URL, e.g. `http://127.0.0.1:8080`) and `config.llm.chat_setup_prompt`.
* A chat reply calls `POST {url}/v1/chat/completions` (OpenAI-style `messages`). The reply text is `choices[0].message.content`. `LLMConfig::chat_setup_message` fills the prompt placeholders: `{sender_name}`, `{sender_username}`, `{sender_back_story}`, `{receiver_name}`, `{receiver_username}`, `{receiver_back_story}`, `{recent_messages}`, `{current_message}`. An empty prompt uses `DEFAULT_CHAT_SETUP_PROMPT`. Older RON files that still say `character_setup_prompt` load into this field.
* Reset re-fetches the current live config without writing.

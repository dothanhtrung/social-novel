# Settings

The admin page (`res/html/admin.html`, `GET /admin`) has a Settings tab next to Users, Sponsored, and Relations.

* Live values come from `ConfigData` (`crates/config`). Save writes the same struct to the RON file at `config_path` (overwrite) and replaces the in-memory config.
* After a successful save the HTTP server restarts so bind address, database pool, `/data` files, and basic-auth enable/disable pick up the new values. Request-time fields (`data_dir` for uploads, `api.per_page`, auth credentials) also update in the `RwLock` immediately.
* Read: `GET /api/admin/config` returns `{config, err}` where `config` is the `Config` struct.
* Save: `POST /api/admin/config` with JSON matching `Config`. Empty data directory, empty listen address, port `0` or above `65535`, or `api.per_page` of `0` are rejected.
* The form shows the postgres `db_path` as Database URL. SQLite settings from the loaded config are sent back unchanged.
* LLM fields are `config.llm.url` (llama.cpp server base URL, e.g. `http://127.0.0.1:8080`) and `config.llm.chat_setup_prompt`.
* A chat reply calls `POST {url}/v1/chat/completions` (OpenAI-style `messages`). The reply text is `choices[0].message.content`. `LLMConfig::chat_setup_message` fills the prompt placeholders: `{sender_name}`, `{sender_username}`, `{sender_back_story}`, `{receiver_name}`, `{receiver_username}`, `{receiver_back_story}`, `{recent_messages}`, `{current_message}`. An empty prompt uses `DEFAULT_CHAT_SETUP_PROMPT`. Older RON files that still say `character_setup_prompt` load into this field.
* Reset re-fetches the current live config without writing.

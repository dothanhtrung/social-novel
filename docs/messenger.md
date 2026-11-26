# Messenger

The messenger page is `res/html/messenger.html` at `GET /messenger`. It does not use the home header.

* The top-right picker is `#messenger-character-picker`. It is not the home `#character-picker`, so it does not list every character and does not change the home main character.
* The list is the open room's members (`GET /api/character?room={id}`), already loaded for the sidebar. The search box filters that list by name or username; it does not call the API again.
* Choosing a row calls `selectMainCharacter`. That updates the header avatar, the member-list highlight, which side each message is on, and two-person room titles. Clicking a sidebar member does the same.

## LLM reply

* The right sidebar, under Chat Members, has an Enable LLM replies checkbox and a Recent messages number (1–50, default 10).
* Changing either field saves `POST /api/room/llm` with JSON `{id, llm_enabled, llm_recent}`. The columns are `chat_room.llm_enabled` and `chat_room.llm_recent` (`db/migrations/postgres/0006_room_llm.sql`). `GET /api/room` returns both, and opening a room fills the controls.
* A new root message in that room (`POST /api/post`, no parent) with non-empty text, when the room flag is on and `config.llm.url` is set, is stored and returned immediately. The response includes `llm_author` (the character who will reply). The page shows the user's message, then a `typing...` row for that character, and polls `GET /api/post/llm/{id}` until the reply is `done` or `failed`. On `done` it reloads the room messages and removes the indicator. A failure removes the indicator and leaves the user's message.
* The receiver is the first other member of the room. The prompt is `config.llm.chat_setup_prompt` (Settings tab), filled with both characters' names, usernames, and back stories, the previous N messages (oldest first, not including the new one), and the new message. The LLM is asked to answer as the receiver.
* The call is `POST {url}/v1/chat/completions`. A failure is logged and the user's message stays. Editing a message does not call the LLM. The reply is inserted directly, not through `POST /api/post`, so it does not trigger another reply.

## Message Actions

* Every message, including attachment-only messages and messages from other characters, has a toolbar above it with a reaction picker and compact 22-pixel edit/delete buttons. It appears on hover or keyboard focus and remains visible on touch devices. Toolbar space is reserved so hovering does not shift messages.
* Editing replaces the message text with an inline textarea and attachment previews. Existing or newly selected attachments can be removed, and the attachment picker accepts multiple images, videos, and audio files. To replace media, remove it and select a new file.
* Save submits text, new `media` files, and repeated `remove_media` IDs to `POST /api/post`. Cancel or Escape discards both text and attachment changes without saving. Authorship, room, and reactions are preserved. A message without retained or new attachments cannot be saved with empty text.
* Delete opens a browser confirmation. Only confirmation sends `GET /api/post/delete/{id}`, and the message disappears only after the API reports success.
* Pending operations disable their controls. Failed edits retain the text and attachment draft for retry; failed deletions keep the message visible. Saved attachments are refreshed from `GET /api/media?post={id}`. Editing and deletion do not reload or scroll the conversation.

## Reactions

* The hover picker offers Like, Love, Haha, Wow, and Sad, matching the existing `liked`, `loved`, `haha`, `surprised`, and `sad` counters.
* As an admin story editor, clicking a picker reaction adds one to that count. Nonzero counts appear as emoji badges beneath the message; clicking a badge removes one, and zero-count badges disappear. These are aggregate story counts, not one reaction per character.
* Changes are saved through `POST /api/post`, preserving text, author, attachments, other reaction counts, and `liked_by`. Counts update only after success, and failed requests leave them unchanged. Controls are disabled while a reaction save is pending. Reactions are unavailable during inline editing.
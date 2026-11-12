# Messenger

The messenger page is `res/html/messenger.html` at `GET /messenger`. It does not use the home header.

* The top-right picker is `#messenger-character-picker`. It is not the home `#character-picker`, so it does not list every character and does not change the home main character.
* The list is the open room's members (`GET /api/character?room={id}`), already loaded for the sidebar. The search box filters that list by name or username; it does not call the API again.
* Choosing a row calls `selectMainCharacter`. That updates the header avatar, the member-list highlight, which side each message is on, and two-person room titles. Clicking a sidebar member does the same.

## Message Actions

* Every message, including attachment-only messages and messages from other characters, has edit and delete icons. They appear on hover or keyboard focus and remain visible on touch devices.
* Editing replaces the message text with an inline textarea and attachment previews. Existing or newly selected attachments can be removed, and the attachment picker accepts multiple images, videos, and audio files. To replace media, remove it and select a new file.
* Save submits text, new `media` files, and repeated `remove_media` IDs to `POST /api/post`. Cancel or Escape discards both text and attachment changes without saving. Authorship, room, and reactions are preserved. A message without retained or new attachments cannot be saved with empty text.
* Delete opens a browser confirmation. Only confirmation sends `GET /api/post/delete/{id}`, and the message disappears only after the API reports success.
* Pending operations disable their controls. Failed edits retain the text and attachment draft for retry; failed deletions keep the message visible. Saved attachments are refreshed from `GET /api/media?post={id}`. Editing and deletion do not reload or scroll the conversation.
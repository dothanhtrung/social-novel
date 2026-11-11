# Messenger

The messenger page is `res/html/messenger.html` at `GET /messenger`. It does not use the home header.

* The top-right picker is `#messenger-character-picker`. It is not the home `#character-picker`, so it does not list every character and does not change the home main character.
* The list is the open room's members (`GET /api/character?room={id}`), already loaded for the sidebar. The search box filters that list by name or username; it does not call the API again.
* Choosing a row calls `selectMainCharacter`. That updates the header avatar, the member-list highlight, which side each message is on, and two-person room titles. Clicking a sidebar member does the same.
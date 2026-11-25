# Spec

Fake Facebook-style social stories for anime characters. One real user (the admin) chooses which character posts, comments, chats, or joins a group.

Detailed behavior and implementation notes are organized by topic in [DESIGN.md](DESIGN.md).

## Characters

* The admin Users tab can set a character back story. That text is only shown and edited on the admin page. It is stored for later use when sending a character to the LLM.

## Relations

* A relation is directed: the from character is that relation of the to character. Example: Himawari is daughter of Hinata.
* The admin page Relations tab lists every relation, or only the relations that touch a chosen character. A user's Relations button opens that filtered list.
* The admin can add a relation, edit its characters and wording, and delete it.
* A character cannot be related to themselves. The wording is required.

## Groups

* The home feed left sidebar ("Your groups") lists only groups the selected character has joined.
* `/group` lists every group, like Facebook's groups page. Each group has a Join button (Leave if that character is already a member).
* Clicking a group opens `/g/{id}` and marks that row as selected (blue background, bold name).
* The group page left sidebar shows that group's avatar, name, description, and member list, plus Join or Leave. The feed stays in the center.
* The group page uses the same Facebook-style layout as the home feed (header, sidebars, composer, post cards), plus a group header: cover, avatar, name, description, and a Discussion tab.
* Edit on the group page changes the group's name, description, avatar, and cover. Leaving an image empty keeps the current file.
* A character profile shows Edit profile only for the selected character. That form changes the avatar, cover, and bio. Leaving an image empty keeps the current file.
* The feed shows root posts whose `group` field equals that group id. Comments stay nested under those posts and are not listed as their own posts.
* Creating a post on a group page stores that group id on the new post.
* Creating a comment (or reply) on a post that belongs to a group stores the same group id. If the client omits it, the API copies the group from the parent post, walking up ancestors if the immediate parent has none.
* Posts created on the home feed (no group selected) are not assigned to a group.
* Chat messages belong to a room, not a group. They are not listed on the home feed, group feeds, or profiles. Messenger and the home friend chat load them with `room={id}`.

## Comments

* Writing a comment or reply on a post adds it under that post without reloading the page.
* The box at the bottom of a post is only for comments on that post.
* Reply opens a box after that comment and its replies, in line with those replies. Sending from it replies to that comment, adds the reply to the chain, and hides the box.
* The comment box is a rounded field with the character avatar. Buttons on the right insert an emoji, attach a photo or video, or post a sticker or GIF.
* Emoji is typed into the comment. A photo or video stays in the box until the comment is sent. A sticker or GIF is posted on its own, as that comment's picture.
* Stickers and GIFs are files on this site. They are not loaded from another service.
* The new-post box and the chat boxes (Messenger and the home friend chat) use the same buttons and the same sticker and GIF files.

## Mentions

* Typing `@` at the start of a post, comment, or chat field (or after a space) suggests characters whose username or name matches the text typed after `@`.
* Choosing a suggestion inserts `@username`. The stored text keeps the username.
* When that text is shown, a known `@username` is the character's name (not the username), linked to `/u/{username}`. An unknown `@token` is left as typed.

## Messenger

* The main character's messages sit on the right. Other members' messages sit on the left. New messages are posted as that character.
* The main character is one of the open chat room's members. The top-right header avatar opens a searchable list of those members. Clicking a row in the right-hand member list selects the same character.
* Search matches name or username. If the chosen character is not in the room, the first member becomes the main character.
* The right sidebar has LLM reply settings for the open room: a checkbox to enable or disable replies, and how many recent messages to send with the new one (1–50). Those values are stored on the room.
* When LLM replies are on and a new root chat message is created, the app sends the configured chat setup prompt to the LLM server. The prompt includes the sender and receiver names, usernames, and back stories, the recent messages, and the new message. The LLM is told to answer as the receiver. The reply is posted in the same room as that receiver. A failed LLM call does not undo the user's message. The other member is the first room member who is not the sender.

## Friends

* Friendship is mutual. A character cannot be friends with themselves.
* The home feed Contacts list is the selected character's friends, not every character. Each row shows the friend's avatar, name, and the latest message from the chat room that contains only those two characters. Group rooms are ignored. A friend with no such room shows "No messages yet".
* Clicking a friend opens a chat at the bottom of the page. Sending a message when that pair has no two-person room creates one, then posts the message as the selected character.
* A profile shows Add Friend when the selected character is not friends with that profile, and Unfriend when they already are. The button is hidden on the selected character's own profile.

## Media

* A media file can belong to more than one post (and comment).
* Uploading a file that is already stored attaches that media to the new post instead of dropping it.
* Deleting a post removes its link. The file is deleted only when no other post still uses that media.
* Deleting a group or chat room does the same for every post in it.

## Settings

* The admin page Settings tab edits the application config: data directory, database URL, listen address and port, API page size, parallel, basic auth, LLM server URL, and chat setup prompt.
* Save writes those values to the RON config file and reloads them into the running application. Leave both auth fields empty to disable basic auth.
* The chat setup prompt is the overall text sent to the LLM for a messenger reply. It can be changed without rebuilding the app. Placeholders are `{sender_name}`, `{sender_username}`, `{sender_back_story}`, `{receiver_name}`, `{receiver_username}`, `{receiver_back_story}`, `{recent_messages}`, and `{current_message}`. An empty prompt uses the built-in default.

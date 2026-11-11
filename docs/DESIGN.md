# Design

The design notes are split by topic:

- [Relations](relations.md)
- [Groups](groups.md)
- [Comments](comments.md)
- [Mentions](mentions.md)
- [Messenger](messenger.md)
- [Friends](friends.md)
- [Media](media.md)


Source code tree
----------------

* crates/api: REST API for all functions.
* crates/config: Handle application config.
* crates/db: Communicate to database (PostgreSQL) using sqlx.
* social-novel: Main application which call other crates.
* res/: Frontend (HTML+JS+CSS) which will get/post data from/to REST API.

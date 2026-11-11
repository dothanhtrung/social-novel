# Media

* `media` stores the file (`url`, `file_type`, `blake3`). `url` is unique and is the content hash plus extension.
* `post_media (post_id, media_id)` links posts to media. One media can be linked to many posts.
* Upload hashes the file. If that path already exists, the existing `media` row is reused and a `post_media` row is inserted (`ON CONFLICT DO NOTHING`).
* Deleting a post deletes its `post_media` rows, then deletes `media` rows (and the file on disk) that have no remaining link. Shared files stay.
* Group and room delete unlinks media the same way before the group or room cascade removes the posts.
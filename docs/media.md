# Media

* `media` stores the file (`url`, `file_type`, `blake3`). `url` is unique and is the content hash plus extension.
* `post_media (post_id, media_id)` links posts to media. One media can be linked to many posts.
* Upload hashes the file. If that path already exists, the existing `media` row is reused and a `post_media` row is inserted (`ON CONFLICT DO NOTHING`).
* Deleting a post deletes its `post_media` rows, then deletes `media` rows (and the file on disk) that have no remaining link. Shared files stay.
* Group and room delete unlinks media the same way before the group or room cascade removes the posts.
* `POST /api/post` accepts optional repeated `remove_media` multipart fields containing media IDs on updates. Only matching links belonging to the edited post are removed; unrelated links and shared files stay. Omitting the field preserves existing attachments.
* New `media` uploads are attached before removals. A successfully reattached media ID is excluded from removal, so replacing an attachment with the same file keeps it. Removals are skipped if any upload fails. As with other post updates, text and successful uploads may already be saved when a later operation fails.
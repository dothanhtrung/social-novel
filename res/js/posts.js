function mediaUrl(m) {
    if (!m || !m.url) return '';
    return (m.url.startsWith('/') || m.url.startsWith('http')) ? m.url : ('/data/' + m.url);
}

function renderMediaHtml(media, wrapperClass) {
    if (!media || media.length === 0) return '';
    const m = media[0];
    const url = mediaUrl(m);
    if (m.file_type === 'Image') {
        return `<div class='${wrapperClass}'><img src="${url}" class='w-full rounded-lg object-cover' alt='media' /></div>`;
    }
    if (m.file_type === 'Video') {
        return `<div class='${wrapperClass}'><video src="${url}" class='plyr-video w-full rounded-lg' playsinline controls></video></div>`;
    }
    return '';
}

function renderPostMeta(p) {
    const feeling = p.feeling || '';
    const is_with = p.is_with || '';
    if (!feeling && !is_with) return '';
    const feelingHtml = feeling ? `<span class='text-fb-blue'>${feeling}</span>` : '';
    let isWithHtml = '';
    if (is_with) {
        const m = is_with.match(/^(with|by|of|to|from)\s+(.*)/i);
        if (m) {
            const prefix = `<span class='text-fb-gray-text'>${m[1]}</span>`;
            const rest = m[2].split(/\s+and\s+/i).map((part, i) => {
                return (i ? `<span class='text-fb-gray-text'> and </span>` : '') + `<span class='text-fb-blue'>${renderContentWithMentions(part)}</span>`;
            }).join('');
            isWithHtml = `${prefix} ${rest}`;
        } else {
            isWithHtml = is_with.split(/\s+and\s+/i).map((part, i) => {
                return (i ? `<span class='text-fb-gray-text'> and </span>` : '') + `<span class='text-fb-blue'>${renderContentWithMentions(part)}</span>`;
            }).join('');
        }
    }
    return `<span class='text-[14px] ml-2 align-middle'><span class='text-fb-gray-text'>is</span> ${feelingHtml}${isWithHtml ? ' ' + isWithHtml : ''}</span>`;
}

function lookupUser(authorId) {
    return (typeof userMap !== 'undefined' && userMap[authorId]) || { username: 'unknown', name: 'Unknown' };
}

function renderCommentHtml(c, depth) {
    const a = lookupUser(c.author);
    const commentMediaHtml = renderMediaHtml(c.media, 'mt-2');
    const indentStyle = depth > 0 ? `style="margin-left:${depth * 24}px"` : '';
    const commentHtml = `
                    <div ${indentStyle} class="pr-4 pl-4 py-3 border-t border-gray-100 bg-white relative" data-comment-id="${c.id}" data-depth="${depth}">
                        <div class="flex items-start gap-3">
                            <div class="size-12 rounded-full bg-cover bg-center" style="background-image: url('${getAvatarUrl(a)}')"></div>
                            <div class="flex-1">

                                <div class="text-[16px] text-[#050505]"><strong><a href="/u/${a.username}">${a.name || a.username}</a></strong></div>

                                        <div class="text-[16px] text-fb-black-text">${renderContentWithMentions(c.content || '')}</div>
                                        ${commentMediaHtml}
                                        <div class="mt-1 flex items-center justify-between">
                                            <div class="flex items-center gap-3">
                                                <p class='text-[13px] text-fb-gray-text' title='${new Date(c.updated_at || c.created_at).toLocaleString()}'>${timeAgo(c.updated_at || c.created_at)}</p>
                                                <button class='text-[13px] text-fb-blue hover:underline reply-btn' data-comment-id='${c.id}'>Reply</button>
                                            </div>
                                            <div>
                                                ${renderReactions(c)}
                                            </div>
                                        </div>
                            </div>
                            <div class='relative ml-2'>
                                <button class='options-btn p-1 rounded-full hover:bg-fb-hover' data-post-id='${c.id}' aria-label='Comment Options'>
                                    <span class='material-symbols-outlined' style='font-size:16px'>more_horiz</span>
                                </button>
                                <div class='options-menu hidden absolute w-28 bg-white rounded shadow-md z-50' style='right:12px; top:8px;'>
                                    <button class='block w-full text-left px-3 py-2 hover:bg-fb-hover options-edit' data-post-id='${c.id}'>Edit</button>
                                    <button class='block w-full text-left px-3 py-2 hover:bg-fb-hover options-delete' data-post-id='${c.id}'>Delete</button>
                                </div>
                            </div>
                        </div>
                    </div>`;

    let childrenHtml = '';
    if (c.comments && c.comments.length > 0) {
        childrenHtml = c.comments.map((child) => renderCommentHtml(child, depth + 1)).join('');
    }
    return commentHtml + childrenHtml;
}

function renderPostHtml(p) {
    const a = lookupUser(p.author);
    const metaHtml = renderPostMeta(p);
    const mediaHtml = renderMediaHtml(p.media, 'w-full mt-2');
    const commentsHtml = (p.comments && p.comments.length > 0)
        ? p.comments.map((c) => renderCommentHtml(c, 0)).join('')
        : '';
    const main = (typeof getMainCharacter === 'function') ? getMainCharacter() : null;
    const mainId = (typeof getMainCharacterId === 'function') ? getMainCharacterId() : '';
    const mainAvatar = main ? getAvatarUrl(main.username) : '';
    return `
                    <div class='bg-white rounded-lg shadow-sm border border-gray-200 mb-4'>
                        <div class='px-4 py-3'>
                            <div class='flex items-center justify-between'>
                                <div class='flex items-center gap-2'>
                                    <div class='size-15 rounded-full bg-cover bg-center' style='background-image: url("${getAvatarUrl(a)}");'></div>
                                    <div>
                                        <div class="flex items-center gap-2">
                                        <a href="/u/${a.username}">
                                            <h4 class='text-[16px] font-bold hover:underline cursor-pointer text-[#050505]'>${a.name || a.username}</h4>
                                        </a>
                                            ${metaHtml}
                                        </div>
                                        <p class='text-[13px] text-fb-gray-text flex items-center gap-1 mt-0'>
                                            ${timeAgo(p.updated_at || p.created_at)} <span class="text-[10px] text-fb-gray-text">•</span> <span class='material-symbols-outlined !text-[14px]'>public</span>
                                        </p>
                                    </div>
                                </div>
                                <div class='relative ml-2'>
                                    <button class='options-btn p-1 rounded-full hover:bg-fb-hover' data-post-id='${p.id}' aria-label='Options'>
                                        <span class='material-symbols-outlined' style='font-size:18px'>more_horiz</span>
                                    </button>
                                    <div class='options-menu hidden absolute w-36 bg-white rounded shadow-md z-50' style='right:12px; top:36px;'>
                                        <button class='block w-full text-left px-3 py-2 hover:bg-fb-hover options-edit' data-post-id='${p.id}'>Edit</button>
                                        <button class='block w-full text-left px-3 py-2 hover:bg-fb-hover options-delete' data-post-id='${p.id}'>Delete</button>
                                    </div>
                                </div>
                            </div>
                            <p class='text-[16px] mt-2 mb-3'>${renderContentWithMentions(p.content || "")}</p>
                        </div>
                        ${mediaHtml}
                        <div class='px-4 pt-2'>
                            <div class='flex items-center gap-2'>
                                ${renderReactions(p)}
                            </div>
                        </div>
                        <div class='mx-4 border-y border-gray-100 flex items-center gap-1 py-1'>
                            <button class='flex-1 flex items-center justify-center gap-2 py-1.5 hover:bg-fb-hover rounded-lg transition-colors text-fb-gray-text'>
                                <span class='material-symbols-outlined !text-[20px]'>thumb_up</span>
                                <span class='text-[15px] font-semibold'>Like</span>
                            </button>
                            <button class='flex-1 flex items-center justify-center gap-2 py-1.5 hover:bg-fb-hover rounded-lg transition-colors text-[#1877F2]'>
                                <span class='material-symbols-outlined !text-[20px] !fill-1'>chat_bubble</span>
                            </button>
                            <button class='flex-1 flex items-center justify-center gap-2 py-1.5 hover:bg-fb-hover rounded-lg transition-colors text-fb-gray-text'>
                                <span class='material-symbols-outlined !text-[20px]'>share</span>
                                <span class='text-[15px] font-semibold'>Share</span>
                            </button>
                        </div>
                        ${commentsHtml}
                        <div class="px-4 py-3 border-t border-gray-100 bg-white">
                            <form class="comment-form flex items-start gap-3" data-parent-id="${p.id}" data-root-id="${p.id}" data-group-id="${p.group || 0}" enctype="multipart/form-data">
                                <input type="hidden" name="id" value="0" />
                                <input type="hidden" name="is_with" value="" />
                                <input type="hidden" name="feeling" value="" />
                                <input type="hidden" name="author" class="main-character-author" value="${mainId}" />
                                <div class="main-character-avatar size-8 rounded-full bg-cover bg-center shrink-0 bg-[#E4E6EB]" style="background-image: url('${mainAvatar}')"></div>
                                <div class="flex-1">
                                    <textarea name="content" placeholder="Write a comment..." class="w-full px-3 py-2 border rounded resize-none" rows="2"></textarea>
                                    <div class="mt-2 flex items-center justify-between">
                                        <div class="flex items-center">
                                            <input type="file" name="media" accept="image/*,video/*" class="hidden comment-media-input" />
                                            <button type="button" class="comment-add-media px-2">📷</button>
                                        </div>
                                    </div>
                                </div>
                            </form>
                        </div>
                    </div>`;
}

function buildPostTree(posts, rootsOnly) {
    const byId = {};
    for (const p of posts) {
        byId[p.id] = { ...p, comments: [] };
    }
    const roots = [];
    for (const id in byId) {
        const p = byId[id];
        const parentId = p.parent && Number(p.parent) > 0 ? Number(p.parent) : 0;
        if (rootsOnly) {
            if (parentId === 0) roots.push(p);
        } else if (parentId && byId[parentId]) {
            byId[parentId].comments.push(p);
        } else {
            roots.push(p);
        }
    }
    return roots;
}

async function fetchCommentTree(post) {
    try {
        const res = await fetch(`/api/post?parent=${post.id}`);
        const data = await res.json().catch(() => ({}));
        const children = data.posts || [];
        children.sort((a, b) => parseTimestamp(a.updated_at || a.created_at) - parseTimestamp(b.updated_at || b.created_at));
        post.comments = children;
        await Promise.all(children.map(fetchCommentTree));
    } catch (e) {
        post.comments = post.comments || [];
    }
}

function collectPostIds(node, out) {
    out.push(node.id);
    if (node.comments && node.comments.length > 0) {
        node.comments.forEach((ch) => collectPostIds(ch, out));
    }
}

async function fetchMediaForPosts(roots) {
    const allPostIds = [];
    roots.forEach((r) => collectPostIds(r, allPostIds));
    const byId = {};
    function index(node) {
        byId[node.id] = node;
        if (node.comments) node.comments.forEach(index);
    }
    roots.forEach(index);
    const uniqIds = Array.from(new Set(allPostIds));
    await Promise.all(uniqIds.map(async (id) => {
        try {
            const mres = await fetch(`/api/media?post=${id}`);
            const mdata = await mres.json().catch(() => ({}));
            if (mdata && Array.isArray(mdata.media) && byId[id]) {
                byId[id].media = mdata.media.map((m) => ({
                    ...m,
                    url: mediaUrl(m),
                }));
            }
        } catch (err) {
            // ignore per-item failures
        }
    }));
}

async function loadAndRenderPosts(options) {
    const container = options.container;
    if (!container) return null;
    const query = options.query instanceof URLSearchParams
        ? options.query
        : new URLSearchParams(options.query || {});
    const rootsOnly = !!options.rootsOnly;
    const emptyHtml = options.emptyHtml || '';

    try {
        const res = await fetch('/api/post?' + query.toString());
        const data = await res.json();
        const posts = data.posts || [];
        if (posts.length === 0) {
            container.innerHTML = emptyHtml;
            return [];
        }

        const roots = buildPostTree(posts, rootsOnly);
        await Promise.all(roots.map(fetchCommentTree));
        await fetchMediaForPosts(roots);
        roots.sort((a, b) => parseTimestamp(b.updated_at || b.created_at) - parseTimestamp(a.updated_at || a.created_at));

        if (roots.length === 0) {
            container.innerHTML = emptyHtml;
            return roots;
        }

        container.innerHTML = roots.map(renderPostHtml).join('');
        initPlyrVideos(container);
        return roots;
    } catch (e) {
        container.innerHTML = '<div class="text-red-500">Failed to load posts</div>';
        return null;
    }
}

function editPost(id) {
    window.location.href = `/p/edit/${id}`;
}

async function deletePost(id) {
    if (!confirm('Are you sure you want to delete this post?')) return;
    try {
        const res = await fetch(`/api/post/delete/${id}`, { method: 'GET' });
        if (!res.ok) {
            console.log('Failed to delete', res.statusText);
        }
        window.location.reload();
    } catch (err) {
        alert('Failed to delete post: ' + err);
    }
}

function initPostOptions() {
    document.addEventListener('click', function (e) {
        if (e.target.closest && e.target.closest('.options-btn')) {
            e.preventDefault();
            const btn = e.target.closest('.options-btn');
            const menu = btn.parentElement.querySelector('.options-menu');
            if (menu) menu.classList.toggle('hidden');
            return;
        }
        if (e.target.closest && e.target.closest('.options-edit')) {
            e.preventDefault();
            const id = e.target.closest('.options-edit').dataset.postId;
            editPost(id);
            return;
        }
        if (e.target.closest && e.target.closest('.options-delete')) {
            e.preventDefault();
            const id = e.target.closest('.options-delete').dataset.postId;
            deletePost(id);
            return;
        }
        if (!e.target.closest || !e.target.closest('.options-menu')) {
            document.querySelectorAll('.options-menu').forEach((m) => m.classList.add('hidden'));
        }
    });
}

function initCommentHandlers() {
    document.addEventListener('click', function (e) {
        const btn = e.target.closest && e.target.closest('.comment-add-media');
        if (btn) {
            e.preventDefault();
            const form = btn.closest('.comment-form');
            if (!form) return;
            const input = form.querySelector('.comment-media-input');
            if (input) input.click();
        }
        const reply = e.target.closest && e.target.closest('.reply-btn');
        if (reply) {
            e.preventDefault();
            const commentId = reply.dataset.commentId;
            let ancestor = reply.closest('.bg-white');
            let form = null;
            while (ancestor) {
                form = ancestor.querySelector && ancestor.querySelector('.comment-form');
                if (form) break;
                ancestor = ancestor.parentElement && ancestor.parentElement.closest
                    ? ancestor.parentElement.closest('.bg-white')
                    : null;
            }
            if (!form) form = document.querySelector('.comment-form');
            if (form) {
                form.dataset.parentId = commentId;
                const ta = form.querySelector('textarea[name="content"]');
                if (ta) ta.focus();
            }
        }
    });

    document.addEventListener('keydown', function (e) {
        const ta = e.target && e.target.closest && e.target.closest('.comment-form') ? e.target : null;
        if (!ta || ta.tagName !== 'TEXTAREA') return;
        if (mentionPopupOpen()) return;
        if (e.key === 'Enter' && !e.shiftKey) {
            e.preventDefault();
            const form = ta.closest('.comment-form');
            if (form) {
                if (typeof form.requestSubmit === 'function') {
                    form.requestSubmit();
                } else {
                    form.dispatchEvent(new Event('submit', { bubbles: true, cancelable: true }));
                }
            }
        }
    });

    document.addEventListener('submit', async function (e) {
        const form = e.target.closest && e.target.closest('.comment-form');
        if (!form) return;
        e.preventDefault();
        if (form.dataset.submitting === '1') return;
        const parentId = form.dataset.parentId || '0';
        const rootId = form.dataset.rootId || parentId;
        const fd = new FormData();
        const author = getMainCharacterId() || form.querySelector('.main-character-author')?.value || '';
        if (!author) {
            alert('Please select a character from the header first.');
            return;
        }
        const content = form.querySelector('textarea[name="content"]')?.value || '';
        fd.set('author', author);
        fd.set('content', content);
        fd.set('is_with', '');
        fd.set('feeling', '');
        fd.set('parent', parentId);
        fd.set('id', '0');
        fd.set('liked', '0');
        fd.set('haha', '0');
        fd.set('loved', '0');
        fd.set('surprised', '0');
        fd.set('sad', '0');
        fd.set('liked_by', '');
        const groupId = Number(form.dataset.groupId || 0);
        fd.set('group', String(Number.isFinite(groupId) && groupId > 0 ? groupId : 0));
        fd.set('room', '0');
        const mediaInput = form.querySelector('.comment-media-input');
        if (mediaInput && mediaInput.files && mediaInput.files.length > 0) {
            for (const f of mediaInput.files) fd.append('media', f);
        }
        form.dataset.submitting = '1';
        try {
            const res = await fetch('/api/post', { method: 'POST', body: fd });
            const data = await res.json().catch(() => ({}));
            if (res.ok && !data.err) {
                const depth = depthForNewComment(form, parentId);
                const card = form.closest('.bg-white.rounded-lg');
                let comment = null;
                if (data.id) {
                    comment = await fetchPostWithMedia(data.id);
                }
                if (!comment) {
                    comment = await findNewChild(parentId, card);
                }
                form.reset();
                form.dataset.parentId = rootId;
                const authorEl = form.querySelector('.main-character-author');
                const authorId = (typeof getMainCharacterId === 'function') ? getMainCharacterId() : '';
                if (authorEl && authorId) authorEl.value = authorId;
                if (comment) {
                    insertCommentNode(form, parentId, comment, depth);
                } else {
                    alert('Comment was saved, but it could not be shown. Refresh to see it.');
                }
                const ta = form.querySelector('textarea[name="content"]');
                if (ta) ta.focus({ preventScroll: true });
            } else {
                alert('Failed to post comment: ' + (data.err || res.statusText));
            }
        } catch (err) {
            alert('Failed to post comment: ' + err);
        } finally {
            delete form.dataset.submitting;
        }
    });
}

function commentDepth(el) {
    const n = Number(el && el.dataset.depth);
    return Number.isFinite(n) && n >= 0 ? n : 0;
}

function depthForNewComment(form, parentId) {
    const rootId = Number(form.dataset.rootId || form.dataset.parentId || 0);
    const parent = Number(parentId);
    if (!parent || parent === rootId) return 0;
    const card = form.closest('.bg-white.rounded-lg');
    const anchor = card && card.querySelector(`[data-comment-id="${parent}"]`);
    return anchor ? commentDepth(anchor) + 1 : 1;
}

async function fetchPostWithMedia(id) {
    const res = await fetch(`/api/post?id=${id}`);
    const data = await res.json().catch(() => ({}));
    const post = data.posts && data.posts[0];
    if (!post) return null;
    post.comments = [];
    try {
        const mres = await fetch(`/api/media?post=${id}`);
        const mdata = await mres.json().catch(() => ({}));
        post.media = (mdata.media || []).map((m) => ({ ...m, url: mediaUrl(m) }));
    } catch (e) {
        post.media = [];
    }
    return post;
}

async function findNewChild(parentId, card) {
    const res = await fetch(`/api/post?parent=${parentId}`);
    const data = await res.json().catch(() => ({}));
    const children = data.posts || [];
    const known = new Set();
    if (card) {
        card.querySelectorAll('[data-comment-id]').forEach((el) => known.add(Number(el.dataset.commentId)));
    }
    const fresh = children.filter((c) => !known.has(Number(c.id)));
    fresh.sort((a, b) => {
        const diff = parseTimestamp(b.updated_at || b.created_at) - parseTimestamp(a.updated_at || a.created_at);
        if (diff !== 0) return diff;
        return Number(b.id) - Number(a.id);
    });
    const post = fresh[0];
    if (!post) return null;
    return fetchPostWithMedia(post.id);
}

function insertCommentNode(form, parentId, comment, depth) {
    const node = document.createElement('div');
    node.innerHTML = renderCommentHtml(comment, depth).trim();
    const el = node.firstElementChild;
    if (!el) return null;
    const card = form.closest('.bg-white.rounded-lg');
    const holder = form.parentElement;
    const rootId = Number(form.dataset.rootId || 0);
    const parent = Number(parentId);
    let anchor = null;
    if (card && parent > 0 && parent !== rootId) {
        anchor = card.querySelector(`[data-comment-id="${parent}"]`);
    }
    if (!holder) return null;
    if (!anchor) {
        holder.before(el);
    } else {
        let after = anchor;
        let next = after.nextElementSibling;
        while (next && next.dataset.commentId && commentDepth(next) > commentDepth(anchor)) {
            after = next;
            next = after.nextElementSibling;
        }
        after.after(el);
    }
    if (typeof initPlyrVideos === 'function') initPlyrVideos(el);
    el.scrollIntoView({ block: 'nearest', behavior: 'smooth' });
    return el;
}

document.addEventListener('DOMContentLoaded', function () {
    initPostOptions();
    initCommentHandlers();
});

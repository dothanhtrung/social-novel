// Shared composer tools for comments, new posts, and chat.
// Stickers and GIFs are local files under /assets. Nothing is fetched from another service.

function commentStickerIcon() {
    return `<svg viewBox="0 0 24 24" width="20" height="20" aria-hidden="true"><path fill="currentColor" d="M5 3h9.2L21 9.8V19a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2zm8 1.8V10h5.2L13 4.8zM8.2 13.1a1.15 1.15 0 1 0 0-2.3 1.15 1.15 0 0 0 0 2.3zm5.6 0a1.15 1.15 0 1 0 0-2.3 1.15 1.15 0 0 0 0 2.3zM8 16.1c.7 1.1 1.9 1.7 4 1.7s3.3-.6 4-1.7"/></svg>`;
}

const COMMENT_EMOJI = [
    ['😀', 'smile happy grin'], ['😁', 'grin'], ['😂', 'laugh joy cry'], ['🤣', 'rofl laugh'],
    ['😊', 'blush smile'], ['😍', 'love heart eyes'], ['😘', 'kiss'], ['😋', 'yum'],
    ['😎', 'cool sunglasses'], ['🤔', 'think'], ['😴', 'sleep tired'], ['😢', 'sad cry'],
    ['😭', 'cry sob'], ['😡', 'angry mad'], ['😱', 'wow scream'], ['🤯', 'mind blown wow'],
    ['👍', 'like thumb up'], ['👎', 'dislike thumb down'], ['👏', 'clap'], ['🙏', 'please thanks'],
    ['👌', 'ok'], ['✌️', 'peace'], ['🤝', 'handshake'], ['💪', 'strong'],
    ['❤️', 'love heart'], ['🧡', 'heart orange'], ['💛', 'heart'], ['💚', 'heart'],
    ['💙', 'heart'], ['💜', 'heart'], ['🖤', 'heart'], ['💔', 'heart broken'],
    ['🔥', 'fire hot'], ['⭐', 'star'], ['✨', 'sparkle'], ['🎉', 'party tada'],
    ['🎂', 'cake birthday'], ['🎁', 'gift'], ['🌹', 'rose flower'], ['🌸', 'flower'],
    ['🍀', 'luck clover'], ['☀️', 'sun'], ['🌙', 'moon night'], ['⚡', 'zap lightning'],
    ['🌈', 'rainbow'], ['🐶', 'dog'], ['🐱', 'cat'], ['🐼', 'panda'],
    ['🦊', 'fox'], ['🍕', 'pizza food'], ['🍔', 'burger food'], ['🍣', 'sushi food'],
    ['🍰', 'cake sweet'], ['☕', 'coffee'], ['🎵', 'music'], ['⚽', 'soccer ball sport'],
    ['🏀', 'basketball sport'], ['🎮', 'game'], ['📚', 'book'], ['📷', 'camera photo'],
    ['✅', 'check ok yes'], ['❌', 'no cross'], ['💯', 'hundred'], ['👀', 'eyes look'],
];

const COMMENT_STICKERS = [
    ['like', 'like thumb up'], ['love', 'love heart'], ['haha', 'haha laugh'], ['wow', 'wow surprised'],
    ['sad', 'sad'], ['angry', 'angry mad'], ['care', 'care hug'], ['cool', 'cool'],
    ['kiss', 'kiss'], ['cry', 'cry sad'], ['party', 'party'], ['sleep', 'sleep tired'],
    ['heart', 'heart love'], ['fire', 'fire hot'], ['star', 'star'], ['ok', 'ok yes'],
    ['clap', 'clap'], ['wave', 'wave hello'],
];

const COMMENT_GIFS = [
    ['like', 'like thumb up'], ['love', 'love heart'], ['haha', 'haha laugh'], ['wow', 'wow surprised'],
    ['sad', 'sad cry'], ['wave', 'wave hello'], ['party', 'party'], ['fire', 'fire hot'],
];

function composerField(form) {
    if (!form) return null;
    return form.querySelector('.comment-input, textarea[name="content"], input[type="text"]');
}

function resizeCommentInput(ta) {
    if (!ta || ta.tagName !== 'TEXTAREA') return;
    ta.style.height = 'auto';
    ta.style.height = Math.min(ta.scrollHeight, 120) + 'px';
}

function setMediaFiles(input, files) {
    const data = new DataTransfer();
    files.forEach((file) => data.items.add(file));
    input.files = data.files;
}

function closeCommentPickers(except) {
    document.querySelectorAll('.comment-picker').forEach((el) => {
        if (el === except) return;
        el.classList.add('hidden');
        el.innerHTML = '';
        el.classList.remove('is-below');
    });
    document.querySelectorAll('.comment-tool.is-open').forEach((btn) => {
        if (except && btn.closest('.comment-box') === except.closest('.comment-box')) return;
        btn.classList.remove('is-open');
    });
}

function packMatches(query) {
    const q = query.trim().toLowerCase();
    return function (item) {
        if (!q) return true;
        return item[0].indexOf(q) !== -1 || item[1].indexOf(q) !== -1;
    };
}

function fillCommentPicker(picker, query) {
    const grid = picker.querySelector('.comment-picker-grid');
    if (!grid) return;
    const kind = picker.dataset.kind;
    const match = packMatches(query || '');
    let html = '';
    if (kind === 'emoji') {
        html = COMMENT_EMOJI.filter(match).map((item) => {
            return `<button type="button" class="comment-picker-item" data-emoji="${item[0]}">${item[0]}</button>`;
        }).join('');
    } else {
        const list = kind === 'gif' ? COMMENT_GIFS : COMMENT_STICKERS;
        const folder = kind === 'gif' ? 'gifs' : 'stickers';
        const ext = kind === 'gif' ? 'gif' : 'png';
        html = list.filter(match).map((item) => {
            const url = `/assets/${folder}/${item[0]}.${ext}`;
            return `<button type="button" class="comment-picker-item" data-url="${url}" data-file="${item[0]}.${ext}"><img src="${url}" alt="${item[0]}"></button>`;
        }).join('');
    }
    grid.innerHTML = html || `<p class="comment-picker-empty">Nothing matches</p>`;
}

function openCommentPicker(btn, kind) {
    const box = btn.closest('.comment-box');
    if (!box) return;
    const picker = box.querySelector('.comment-picker');
    if (!picker) return;
    const already = btn.classList.contains('is-open');
    closeCommentPickers();
    if (already) return;
    const placeholder = kind === 'emoji' ? 'Search emoji' : (kind === 'gif' ? 'Search GIFs' : 'Search stickers');
    picker.dataset.kind = kind;
    picker.innerHTML = `<input type="text" class="comment-picker-search" placeholder="${placeholder}" />
        <div class="comment-picker-grid ${kind === 'emoji' ? 'is-emoji' : 'is-pack'}"></div>`;
    fillCommentPicker(picker, '');
    picker.classList.remove('hidden');
    btn.classList.add('is-open');
    const top = picker.getBoundingClientRect().top;
    if (top < 8) picker.classList.add('is-below');
    const search = picker.querySelector('.comment-picker-search');
    if (search) search.focus();
}

function insertCommentEmoji(form, emoji) {
    const ta = composerField(form);
    if (!ta || !emoji) return;
    const start = ta.selectionStart || 0;
    const end = ta.selectionEnd || 0;
    ta.value = ta.value.slice(0, start) + emoji + ta.value.slice(end);
    const caret = start + emoji.length;
    ta.focus();
    ta.setSelectionRange(caret, caret);
    resizeCommentInput(ta);
}

function showCommentAttachPreview(input) {
    const box = input.closest('.comment-box') || input.closest('form');
    const preview = box && box.querySelector('.comment-attach-preview');
    if (!preview) return;
    preview.innerHTML = '';
    const files = input.files ? Array.from(input.files) : [];
    files.forEach((file, index) => {
        const url = URL.createObjectURL(file);
        const chip = document.createElement('div');
        chip.className = 'comment-attach-chip';
        const media = document.createElement(file.type.indexOf('video/') === 0 ? 'video' : 'img');
        media.src = url;
        chip.appendChild(media);
        const remove = document.createElement('button');
        remove.type = 'button';
        remove.className = 'comment-attach-remove';
        remove.setAttribute('aria-label', 'Remove attachment');
        remove.dataset.index = String(index);
        remove.textContent = '×';
        chip.appendChild(remove);
        preview.appendChild(chip);
    });
}

function removeCommentAttach(button) {
    const form = button.closest('form');
    const input = form && form.querySelector('.comment-media-input');
    if (!input) return;
    const index = Number(button.dataset.index);
    const kept = Array.from(input.files || []).filter((_, i) => i !== index);
    setMediaFiles(input, kept);
    showCommentAttachPreview(input);
}

async function postCommentPack(form, url, filename) {
    const res = await fetch(url);
    if (!res.ok) throw new Error(res.statusText || 'missing file');
    const blob = await res.blob();
    const type = blob.type || (filename.endsWith('.gif') ? 'image/gif' : 'image/png');
    const file = new File([blob], filename, { type: type });
    const input = form.querySelector('.comment-media-input');
    if (!input) return;
    setMediaFiles(input, [file]);
    const ta = composerField(form);
    if (ta) {
        ta.value = '';
        resizeCommentInput(ta);
    }
    if (typeof form.requestSubmit === 'function') {
        form.requestSubmit();
    } else {
        form.dispatchEvent(new Event('submit', { bubbles: true, cancelable: true }));
    }
}

function initComposerTools() {
    document.addEventListener('mousedown', function (e) {
        const keep = e.target.closest && e.target.closest('.comment-picker-item, .comment-tool, .comment-attach-remove, .reply-cancel');
        if (keep && !(e.target.closest && e.target.closest('.comment-picker-search'))) e.preventDefault();
    });

    document.addEventListener('click', function (e) {
        const picked = e.target.closest && e.target.closest('.comment-picker-item');
        if (picked) {
            e.preventDefault();
            const form = picked.closest('form');
            if (!form) return;
            if (picked.dataset.emoji) {
                insertCommentEmoji(form, picked.dataset.emoji);
                return;
            }
            if (picked.dataset.url) {
                closeCommentPickers();
                postCommentPack(form, picked.dataset.url, picked.dataset.file).catch((err) => {
                    alert('Could not add that: ' + err);
                });
            }
            return;
        }
        const sticker = e.target.closest && e.target.closest('.comment-sticker');
        if (sticker) {
            e.preventDefault();
            openCommentPicker(sticker, 'sticker');
            return;
        }
        const gif = e.target.closest && e.target.closest('.comment-gif');
        if (gif) {
            e.preventDefault();
            openCommentPicker(gif, 'gif');
            return;
        }
        const emoji = e.target.closest && e.target.closest('.comment-emoji');
        if (emoji) {
            e.preventDefault();
            openCommentPicker(emoji, 'emoji');
            return;
        }
        const remove = e.target.closest && e.target.closest('.comment-attach-remove');
        if (remove) {
            e.preventDefault();
            removeCommentAttach(remove);
            return;
        }
        const btn = e.target.closest && e.target.closest('.comment-add-media');
        if (btn) {
            e.preventDefault();
            closeCommentPickers();
            const form = btn.closest('form');
            const input = form && form.querySelector('.comment-media-input');
            if (input) input.click();
            return;
        }
        if (!e.target.closest || !e.target.closest('.comment-picker, .comment-tool')) {
            closeCommentPickers();
        }
    });

    document.addEventListener('input', function (e) {
        const ta = e.target && e.target.classList && e.target.classList.contains('comment-input') ? e.target : null;
        if (ta) resizeCommentInput(ta);
        const search = e.target && e.target.classList && e.target.classList.contains('comment-picker-search') ? e.target : null;
        if (search) fillCommentPicker(search.closest('.comment-picker'), search.value);
    });

    document.addEventListener('change', function (e) {
        const input = e.target && e.target.classList && e.target.classList.contains('comment-media-input') ? e.target : null;
        if (input) showCommentAttachPreview(input);
    });

    document.addEventListener('keydown', function (e) {
        const search = e.target && e.target.classList && e.target.classList.contains('comment-picker-search') ? e.target : null;
        if (!search) return;
        if (e.key === 'Escape') {
            e.preventDefault();
            e.stopPropagation();
            closeCommentPickers();
        }
        if (e.key === 'Enter') {
            e.preventDefault();
            e.stopPropagation();
        }
    });
}

document.addEventListener('DOMContentLoaded', initComposerTools);


function isVideoMedia(url) {
    const path = String(url || '').split('?')[0].toLowerCase();
    return /\.(mp4|webm|mov|m4v|ogv|ogg)$/.test(path);
}

function getAvatarUrl(user) {
    const username = typeof user === 'string' ? user : (user && user.username);
    if (username) {
        return `/data/avatar/${username}.png`;
    }
    return '';
}

function getCoverUrl(user) {
    const username = typeof user === 'string' ? user : (user && user.username);
    if (username) {
        return `/data/cover/${username}.png`;
    }
    return '';
}

const MAIN_CHARACTER_KEY = 'mainCharacterId';
let mainCharacter = null;
let characterListCache = [];

function getMainCharacter() {
    return mainCharacter;
}

function getMainCharacterId() {
    return mainCharacter ? mainCharacter.id : '';
}

function applyMainCharacter(user) {
    mainCharacter = user || null;
    if (user && user.id != null) {
        try {
            localStorage.setItem(MAIN_CHARACTER_KEY, String(user.id));
        } catch (e) {
            // ignore storage failures (private mode, etc.)
        }
    }

    const avatarUrl = user ? getAvatarUrl(user.username) : '';
    const headerAvatar = document.getElementById('user-selection');
    if (headerAvatar) {
        headerAvatar.style.backgroundImage = avatarUrl ? `url('${avatarUrl}')` : '';
        headerAvatar.title = user ? (user.name || user.username || '') : 'Select character';
        headerAvatar.setAttribute('aria-label', user ? (user.name || user.username) : 'Select character');
    }

    document.querySelectorAll('.main-character-avatar').forEach((el) => {
        el.style.backgroundImage = avatarUrl ? `url('${avatarUrl}')` : '';
    });
    document.querySelectorAll('.main-character-author').forEach((el) => {
        el.value = user ? user.id : '';
    });
    document.querySelectorAll('.main-character-name').forEach((el) => {
        el.textContent = user ? (user.name || user.username || '') : '';
    });

    document.dispatchEvent(new CustomEvent('main-character-change', { detail: user }));
}

async function fetchCharacterList(search) {
    const params = new URLSearchParams();
    if (search) params.set('search', search);
    const qs = params.toString();
    const res = await fetch('/api/character' + (qs ? '?' + qs : ''));
    const data = await res.json();
    return data.characters || [];
}

function renderCharacterDropdownList(characters, query) {
    const list = document.getElementById('character-dropdown-list');
    if (!list) return;
    const q = (query || '').trim().toLowerCase();
    const filtered = q
        ? characters.filter((c) => (c.username || '').toLowerCase().includes(q))
        : characters;
    if (filtered.length === 0) {
        list.innerHTML = '<div class="px-3 py-4 text-sm text-[#65676B]">No characters found</div>';
        return;
    }
    const selectedId = mainCharacter ? Number(mainCharacter.id) : null;
    list.innerHTML = filtered.map((c) => {
        const selected = Number(c.id) === selectedId;
        const name = escapeHtml(c.name || c.username || '');
        const username = escapeHtml(c.username || '');
        return `<button type="button" class="character-option w-full flex items-center gap-3 px-3 py-2 hover:bg-[#E4E6EB] text-left ${selected ? 'bg-[#E7F3FF]' : ''}" data-character-id="${c.id}">
            <div class="size-9 rounded-full bg-cover bg-center shrink-0 bg-[#E4E6EB]" style="background-image: url('${getAvatarUrl(c.username)}')"></div>
            <div class="min-w-0 flex-1">
                <div class="font-semibold text-[15px] truncate">${name}</div>
                <div class="text-[13px] text-[#65676B] truncate">@${username}</div>
            </div>
        </button>`;
    }).join('');
}

function setCharacterDropdownOpen(open) {
    const dropdown = document.getElementById('character-dropdown');
    const trigger = document.getElementById('user-selection');
    if (!dropdown) return;
    dropdown.classList.toggle('hidden', !open);
    if (trigger) trigger.setAttribute('aria-expanded', open ? 'true' : 'false');
}

function initMainCharacterPicker() {
    const picker = document.getElementById('character-picker');
    const trigger = document.getElementById('user-selection');
    const dropdown = document.getElementById('character-dropdown');
    const search = document.getElementById('character-search');
    const list = document.getElementById('character-dropdown-list');
    if (!picker || !trigger || !dropdown) return;

    trigger.addEventListener('keydown', (e) => {
        if (e.key === 'Enter' || e.key === ' ') {
            e.preventDefault();
            trigger.click();
        }
    });

    trigger.addEventListener('click', async (e) => {
        e.stopPropagation();
        const willOpen = dropdown.classList.contains('hidden');
        if (willOpen) {
            setCharacterDropdownOpen(true);
            try {
                characterListCache = await fetchCharacterList();
            } catch (err) {
                characterListCache = characterListCache || [];
            }
            if (search) search.value = '';
            renderCharacterDropdownList(characterListCache, '');
            if (search) search.focus();
        } else {
            setCharacterDropdownOpen(false);
        }
    });

    if (search) {
        search.addEventListener('input', () => {
            renderCharacterDropdownList(characterListCache || [], search.value);
        });
        search.addEventListener('keydown', (e) => {
            if (e.key === 'Escape') {
                setCharacterDropdownOpen(false);
                trigger.focus();
            } else if (e.key === 'Enter') {
                e.preventDefault();
                const first = list && list.querySelector('.character-option');
                if (first) first.click();
            }
        });
    }

    if (list) {
        list.addEventListener('click', (e) => {
            const opt = e.target.closest('.character-option');
            if (!opt) return;
            const id = Number(opt.dataset.characterId);
            const user = (characterListCache || []).find((c) => Number(c.id) === id);
            if (user) applyMainCharacter(user);
            setCharacterDropdownOpen(false);
        });
    }

    document.addEventListener('click', (e) => {
        if (!dropdown.classList.contains('hidden') && !picker.contains(e.target)) {
            setCharacterDropdownOpen(false);
        }
    });

    document.addEventListener('keydown', (e) => {
        if (e.key === 'Escape' && !dropdown.classList.contains('hidden')) {
            setCharacterDropdownOpen(false);
        }
    });
}

async function initMainCharacter() {
    const trigger = document.getElementById('user-selection');
    if (!trigger) return;
    initMainCharacterPicker();
    try {
        characterListCache = await fetchCharacterList();
        let savedId = null;
        try {
            savedId = localStorage.getItem(MAIN_CHARACTER_KEY);
        } catch (e) {
            savedId = null;
        }
        let user = null;
        if (savedId) {
            user = characterListCache.find((c) => String(c.id) === String(savedId));
        }
        if (!user && characterListCache.length > 0) {
            user = characterListCache[0];
        }
        if (user) applyMainCharacter(user);
    } catch (e) {
        console.error('Failed to load characters', e);
    }
}

document.addEventListener('DOMContentLoaded', initMainCharacter);

// Escape HTML special chars to prevent XSS when rendering user content
function escapeHtml(str) {
    return String(str)
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;')
        .replace(/"/g, '&quot;')
        .replace(/'/g, '&#39;');
}


// Robust parser for backend timestamp strings like:
// "2026-01-16 8:40:12.33977 +00:00:00"
function parseTimestamp(s) {
    if (!s) return new Date();
    let str = String(s).trim();
    // Replace first space between date and time with 'T'
    str = str.replace(/^([0-9]{4}-[0-9]{2}-[0-9]{2})\s+/, '$1T');
    // Remove any extra space before timezone
    str = str.replace(/\s+([+-]\d{2}:\d{2}:?\d{0,2})$/, '$1');
    // Convert timezone like +00:00:00 to +00:00 (drop seconds)
    str = str.replace(/([+-]\d{2}:\d{2}):\d{2}$/, '$1');
    // Ensure hour has two digits (e.g. T8: -> T08:)
    str = str.replace(/T(\d):/, 'T0$1:');
    // If there's no timezone or Z, assume UTC
    if (!/[Zz]|[+-]\d{2}:\d{2}$/.test(str)) {
        str = str + 'Z';
    }
    const d = new Date(str);
    if (isNaN(d.getTime())) {
        // Try fallback by removing fractional seconds
        const alt = String(s).replace(/(\.[0-9]+)\s+/, ' ');
        const d2 = new Date(alt);
        if (!isNaN(d2.getTime())) return d2;
        return new Date(s);
    }
    return d;
}

// Friendly relative time like Facebook: few seconds ago, 15 minutes ago, 1 day ago, or 'Jan 16 at 17:04'
function timeAgo(s) {
    const d = parseTimestamp(s);
    const now = new Date();
    const diff = Math.floor((now - d) / 1000); // seconds
    if (diff < 10) return 'few seconds ago';
    if (diff < 60) return `${diff} seconds ago`;
    if (diff < 3600) {
        const m = Math.floor(diff / 60);
        return `${m} minute${m !== 1 ? 's' : ''} ago`;
    }
    if (diff < 86400) {
        const h = Math.floor(diff / 3600);
        return `${h} hour${h !== 1 ? 's' : ''} ago`;
    }
    if (diff < 604800) { // less than 7 days
        const days = Math.floor(diff / 86400);
        return `${days} day${days !== 1 ? 's' : ''} ago`;
    }
    // older: if same year, show 'Jan 16 at 17:04', else full date
    const nowYear = now.getFullYear();
    if (d.getFullYear() === nowYear) {
        return d.toLocaleString(undefined, { month: 'short', day: 'numeric' }) + ' at ' + d.toLocaleTimeString(undefined, { hour: '2-digit', minute: '2-digit' });
    }
    return d.toLocaleString();
}

// Parse page and count from URL query string, with defaults
function getPageParams() {
    const params = new URLSearchParams(window.location.search);
    const page = Math.max(1, Number(params.get('page') || 1));
    const count = Math.max(1, Number(params.get('count') || 20));
    return { page, count };
}

// Update browser URL to reflect current page/count without reloading
function updateUrlParams(page, count) {
    const params = new URLSearchParams(window.location.search);
    params.set('page', String(page));
    params.set('count', String(count));
    const newUrl = window.location.pathname + '?' + params.toString();
    window.history.pushState({}, '', newUrl);
}


// Characters for @mentions. Pages that already load /api/character can call
// rememberCharacters(); otherwise the suggester loads them on first @.
let mentionCharacters = [];
let mentionUsernameMap = {};
let mentionLoad = null;

function rememberCharacters(characters) {
    const byId = {};
    const extras = [];
    function add(c) {
        if (!c) return;
        if (c.id != null) byId[c.id] = c;
        else extras.push(c);
        if (c.username) mentionUsernameMap[c.username.toLowerCase()] = c;
    }
    mentionCharacters.forEach(add);
    (characters || []).forEach(add);
    mentionCharacters = Object.values(byId).concat(extras);
    if (typeof usernameMapLower !== 'undefined') {
        Object.assign(usernameMapLower, mentionUsernameMap);
    }
}

function lookupMention(uname) {
    const key = String(uname || '').toLowerCase();
    if (typeof usernameMapLower !== 'undefined' && usernameMapLower[key]) return usernameMapLower[key];
    return mentionUsernameMap[key] || null;
}

async function ensureMentionCharacters() {
    if (typeof characterListCache !== 'undefined' && characterListCache.length > 0) {
        rememberCharacters(characterListCache);
    }
    // Always fetch the full list once. A room page may have remembered only its members.
    if (mentionLoad) return mentionLoad;
    mentionLoad = fetchCharacterList().then((list) => {
        rememberCharacters(list);
        return mentionCharacters;
    }).catch((err) => {
        mentionLoad = null;
        console.error('Failed to load characters for mentions', err);
        return mentionCharacters;
    });
    return mentionLoad;
}

function filterMentionCharacters(query) {
    const q = (query || '').toLowerCase();
    const matches = mentionCharacters.filter((c) => {
        const username = (c.username || '').toLowerCase();
        const name = (c.name || '').toLowerCase();
        if (!username) return false;
        if (!q) return true;
        return username.startsWith(q) || name.startsWith(q) || username.includes(q);
    });
    matches.sort((a, b) => {
        const au = (a.username || '').toLowerCase();
        const bu = (b.username || '').toLowerCase();
        const aPrefix = au.startsWith(q) ? 0 : 1;
        const bPrefix = bu.startsWith(q) ? 0 : 1;
        if (aPrefix !== bPrefix) return aPrefix - bPrefix;
        return au.localeCompare(bu);
    });
    return matches.slice(0, 8);
}

// @ is a mention only at the start of the field or after whitespace.
function mentionQueryAt(value, caret) {
    const before = String(value || '').slice(0, caret);
    const at = before.lastIndexOf('@');
    if (at < 0) return null;
    if (at > 0 && !/\s/.test(before.charAt(at - 1))) return null;
    const query = before.slice(at + 1);
    if (/\s/.test(query) || query.length > 40) return null;
    return { start: at, query };
}

// Render content and replace @username with the character name, linked to /u/{username}.
function renderContentWithMentions(text) {
    if (!text) return '';
    // Escape first
    let out = escapeHtml(text);
    // Replace newlines with <br>
    out = out.replace(/\r?\n/g, '<br>');
    out = out.replace(/@([A-Za-z0-9_\-\.]+)/g, function(_, uname) {
        const found = lookupMention(uname);
        if (found && found.username) {
            const display = escapeHtml(found.name || found.username);
            const href = '/u/' + encodeURIComponent(found.username);
            return `<a class="mention-link" href="${href}">${display}</a>`;
        }
        return `@${escapeHtml(uname)}`;
    });
    return out;
}

// Dropdown shown while typing @username in a post, comment, or chat field.
function ensureMentionPopup() {
    let popup = document.getElementById('mention-suggest');
    if (popup) return popup;
    popup = document.createElement('div');
    popup.id = 'mention-suggest';
    popup.className = 'mention-suggest hidden';
    popup.setAttribute('role', 'listbox');
    document.body.appendChild(popup);
    popup.addEventListener('mousedown', (e) => {
        // Keep focus in the field so the caret (and the insertion point) stays put.
        e.preventDefault();
        const opt = e.target.closest('.mention-option');
        if (!opt || !mentionTarget) return;
        acceptMention(mentionTarget, Number(opt.dataset.index));
    });
    return popup;
}

let mentionTarget = null;
let mentionMatches = [];
let mentionActive = 0;
let mentionRange = null;

function hideMentionPopup() {
    const popup = document.getElementById('mention-suggest');
    if (popup) {
        popup.classList.add('hidden');
        popup.innerHTML = '';
    }
    mentionTarget = null;
    mentionMatches = [];
    mentionRange = null;
}

function renderMentionPopup() {
    const popup = ensureMentionPopup();
    if (!mentionTarget || mentionMatches.length === 0) {
        hideMentionPopup();
        return;
    }
    popup.innerHTML = mentionMatches.map((c, i) => {
        const name = escapeHtml(c.name || c.username || '');
        const username = escapeHtml(c.username || '');
        return `<button type="button" role="option" class="mention-option${i === mentionActive ? ' is-active' : ''}" data-index="${i}" aria-selected="${i === mentionActive}">
            <span class="mention-avatar" style="background-image: url('${getAvatarUrl(c.username)}')"></span>
            <span class="mention-text">
                <span class="mention-name">${name}</span>
                <span class="mention-username">@${username}</span>
            </span>
        </button>`;
    }).join('');
    const rect = mentionTarget.getBoundingClientRect();
    const width = Math.min(320, Math.max(220, rect.width));
    let left = rect.left;
    if (left + width > window.innerWidth - 8) left = Math.max(8, window.innerWidth - width - 8);
    popup.style.width = width + 'px';
    popup.style.left = left + 'px';
    popup.classList.remove('hidden');
    const height = popup.offsetHeight;
    let top = rect.bottom + 4;
    if (top + height > window.innerHeight - 8) top = Math.max(8, rect.top - height - 4);
    popup.style.top = top + 'px';
    const active = popup.querySelector('.is-active');
    if (active) active.scrollIntoView({ block: 'nearest' });
}

function acceptMention(el, index) {
    const c = mentionMatches[index];
    const range = mentionRange;
    if (!c || !c.username || !range || !el) {
        hideMentionPopup();
        return;
    }
    const value = el.value;
    const insert = '@' + c.username + ' ';
    el.value = value.slice(0, range.start) + insert + value.slice(el.selectionStart);
    const caret = range.start + insert.length;
    el.setSelectionRange(caret, caret);
    el.focus();
    hideMentionPopup();
    el.dispatchEvent(new Event('input', { bubbles: true }));
}

function mentionPopupOpen() {
    const popup = document.getElementById('mention-suggest');
    return !!(popup && !popup.classList.contains('hidden') && mentionMatches.length > 0);
}

async function updateMentionPopup(el) {
    if (!el || (el.tagName !== 'TEXTAREA' && el.tagName !== 'INPUT')) {
        hideMentionPopup();
        return;
    }
    const range = mentionQueryAt(el.value, el.selectionStart);
    if (!range) {
        hideMentionPopup();
        return;
    }
    await ensureMentionCharacters();
    // Caret may have moved while characters were loading.
    if (document.activeElement !== el) return;
    const still = mentionQueryAt(el.value, el.selectionStart);
    if (!still || still.start !== range.start) return;
    const matches = filterMentionCharacters(still.query);
    if (matches.length === 0) {
        hideMentionPopup();
        return;
    }
    mentionTarget = el;
    mentionRange = still;
    mentionMatches = matches;
    if (mentionActive >= matches.length) mentionActive = 0;
    renderMentionPopup();
}

// Post, comment, and chat composers. Not search boxes or group fields.
function isMentionField(el) {
    if (!el || !el.matches) return false;
    if (el.id === 'composer-content' || el.id === 'post-content') return true;
    return el.matches('textarea[name="content"]');
}

function bindMentionInput(el) {
    if (!isMentionField(el) || el.dataset.mentionBound === '1') return;
    el.dataset.mentionBound = '1';
    el.addEventListener('input', () => {
        mentionActive = 0;
        updateMentionPopup(el);
    });
    el.addEventListener('click', () => updateMentionPopup(el));
    el.addEventListener('keyup', (e) => {
        if (e.key === 'ArrowUp' || e.key === 'ArrowDown' || e.key === 'Enter' || e.key === 'Tab' || e.key === 'Escape') return;
        updateMentionPopup(el);
    });
    el.addEventListener('keydown', (e) => {
        if (!mentionPopupOpen() || mentionTarget !== el) return;
        if (e.key === 'ArrowDown') {
            e.preventDefault();
            mentionActive = (mentionActive + 1) % mentionMatches.length;
            renderMentionPopup();
        } else if (e.key === 'ArrowUp') {
            e.preventDefault();
            mentionActive = (mentionActive - 1 + mentionMatches.length) % mentionMatches.length;
            renderMentionPopup();
        } else if ((e.key === 'Enter' && !e.shiftKey) || e.key === 'Tab') {
            e.preventDefault();
            e.stopPropagation();
            acceptMention(el, mentionActive);
        } else if (e.key === 'Escape') {
            e.preventDefault();
            e.stopPropagation();
            hideMentionPopup();
        }
    });
    el.addEventListener('blur', () => {
        // mousedown on an option prevents this blur; a real blur should close the list.
        setTimeout(() => {
            if (mentionTarget === el && document.activeElement !== el) hideMentionPopup();
        }, 0);
    });
}

function initMentionSuggestions(root) {
    const scope = root || document;
    scope.querySelectorAll('textarea[name="content"], #post-content, #composer-content').forEach(bindMentionInput);
}

document.addEventListener('focusin', (e) => {
    bindMentionInput(e.target);
});

document.addEventListener('DOMContentLoaded', () => {
    initMentionSuggestions();
    ensureMentionCharacters();
});

window.addEventListener('scroll', () => {
    if (mentionTarget) renderMentionPopup();
}, true);
window.addEventListener('resize', () => {
    if (mentionTarget) renderMentionPopup();
});


// Render reactions with overlapped icons and total count
function renderReactions(p) {
    const items = [];
    let total = 0;
    if (p.liked && p.liked > 0) { items.push({ color: '', emoji: '👍', count: p.liked }); total += Number(p.liked); }
    if (p.loved && p.loved > 0) { items.push({ color: '', emoji: '❤️', count: p.loved }); total += Number(p.loved); }
    if (p.haha && p.haha > 0) { items.push({ color: '', emoji: '😂', count: p.haha }); total += Number(p.haha); }
    if (p.surprised && p.surprised > 0) { items.push({ color: '', emoji: '😮', count: p.surprised }); total += Number(p.surprised); }
    if (p.sad && p.sad > 0) { items.push({ color: '', emoji: '😢', count: p.sad }); total += Number(p.sad); }
    if (items.length === 0) return '';
    const icons = items.slice(0,3).map((it, idx) => `<span class='reaction-icon' style='background:${it.color}; z-index:${10 - idx}; margin-left:${idx === 0 ? 0 : -6}px'>${it.emoji}</span>`).join('');

  let totalHtml = (total-1) === 0 ? '' : `and <span class='text-fb-blue'>${ total - 1 } others</span>`;
  
    let likedByHtml = p.liked_by.split(',').map((part, i) => {
        return (i ? `<span class='text-fb-gray-text'>, </span>` : '') + `${renderContentWithMentions(part)}`;
    }).join('');
    return `<span class='reaction-stack'>${icons}<span class='reaction-count'>by ${likedByHtml} ${totalHtml}</span></span>`;
}

// Initialize Plyr on all video elements with class 'plyr-video' that haven't been initialized yet
function initPlyrVideos(container) {
    const root = container || document;
    const videos = root.querySelectorAll('video.plyr-video:not(.plyr--setup)');
    videos.forEach(function(v) {
        v.classList.add('plyr--setup');
        new Plyr(v, {
            controls: ['play-large', 'play', 'progress', 'current-time', 'mute', 'volume', 'fullscreen'],
            resetOnEnd: true,
        });
    });
}

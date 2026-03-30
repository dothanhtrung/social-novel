
function getAvatarUrl(user) {
    if (user && user.username) {
        return `/data/avatar/${user.username}.png`;
    }
}

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

// Render reactions with overlapped icons and total count
function renderReactions(p) {
    const items = [];
    let total = 0;
    if (p.liked && p.liked > 0) { items.push({ color: '#1877F2', emoji: '👍', count: p.liked }); total += Number(p.liked); }
    if (p.loved && p.loved > 0) { items.push({ color: '#F3425E', emoji: '❤️', count: p.loved }); total += Number(p.loved); }
    if (p.haha && p.haha > 0) { items.push({ color: '#F7B500', emoji: '😂', count: p.haha }); total += Number(p.haha); }
    if (p.surprised && p.surprised > 0) { items.push({ color: '#9B8CFF', emoji: '😮', count: p.surprised }); total += Number(p.surprised); }
    if (p.sad && p.sad > 0) { items.push({ color: '#6E6E6E', emoji: '😢', count: p.sad }); total += Number(p.sad); }
    if (items.length === 0) return '';
    const icons = items.slice(0,3).map((it, idx) => `<span class='reaction-icon' style='background:${it.color}; z-index:${10 - idx}; margin-left:${idx === 0 ? 0 : -6}px'>${it.emoji}</span>`).join('');
    return `<span class='reaction-stack'>${icons}<span class='reaction-count'>${total}</span></span>`;
}

// Render reactions for comments: if none, render empty string; otherwise overlapped icons + total
function renderCommentReactions(p) {
    const items = [];
    let total = 0;
    if (p.liked && p.liked > 0) { items.push({ color: '#1877F2', emoji: '👍', count: p.liked }); total += Number(p.liked); }
    if (p.loved && p.loved > 0) { items.push({ color: '#F3425E', emoji: '❤️', count: p.loved }); total += Number(p.loved); }
    if (p.haha && p.haha > 0) { items.push({ color: '#F7B500', emoji: '😂', count: p.haha }); total += Number(p.haha); }
    if (p.surprised && p.surprised > 0) { items.push({ color: '#9B8CFF', emoji: '😮', count: p.surprised }); total += Number(p.surprised); }
    if (p.sad && p.sad > 0) { items.push({ color: '#6E6E6E', emoji: '😢', count: p.sad }); total += Number(p.sad); }
    if (items.length === 0) return '';
    // show up to 3 icons overlapped
    const icons = items.slice(0,3).map((it, idx) => `<span class='reaction-icon' style='background:${it.color}; z-index:${10 - idx}; margin-left:${idx === 0 ? 0 : -6}px'>${it.emoji}</span>`).join('');
    return `<span class='reaction-stack'>${icons}<span class='reaction-count'>${total}</span></span>`;
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

// Render content and replace @username mentions with full Name in blue (requires usernameMapLower in global scope)
function renderContentWithMentions(text) {
    if (!text) return '';
    // Escape first
    let out = escapeHtml(text);
    // Replace newlines with <br>
    out = out.replace(/\r?\n/g, '<br>');
    // Replace @username (alphanumeric and underscore, hyphen allowed) with full name when available
    out = out.replace(/@([A-Za-z0-9_\-\.]+)/g, function(_, uname) {
        const found = usernameMapLower[uname.toLowerCase()];
        if (found) {
            const display = escapeHtml(found.name || found.username);
            return `<span class="text-fb-blue">@${display}</span>`;
        }
        return `@${escapeHtml(uname)}`;
    });
    return out;
}


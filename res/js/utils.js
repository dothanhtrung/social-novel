
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

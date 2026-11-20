let comment_characters = [];

// Load characters
async function loadCharacters() {
    try {
        const res = await fetch("/api/character");
        const data = await res.json();
        if (data.characters && Array.isArray(data.characters)) {
            comment_characters = data.characters;
            populateCharacterDropdown();
        }
    } catch (err) {
        console.error("Failed to load characters:", err);
    }
}

// Populate character dropdown
function populateCharacterDropdown() {
    const dropdown = document.getElementById('character-dropdown');
    dropdown.innerHTML = '';

    comment_characters.forEach(char => {
        const option = document.createElement('div');
        option.className = 'flex items-center p-2 hover:bg-gray-100 cursor-pointer';
        option.innerHTML = `
            <img src="/data/avatar/${char.username}.png" alt="${char.name}" class="w-8 h-8 rounded-full mr-3 object-cover">
            <div>
                <div class="font-medium text-sm">${char.name}</div>
                <div class="text-gray-500 text-xs">@${char.username}</div>
            </div>
        `;

        option.addEventListener('click', () => {
            selectCharacter(char);
            toggleCharacterDropdown(false);
        });

        dropdown.appendChild(option);
    });
}

// Select character
function selectCharacter(char) {
    document.querySelector('input[name="author"]').value = char.id;
    document.getElementById('selected-avatar').src = `/data/avatar/${char.username}.png`;
    document.getElementById('selected-avatar').alt = char.name;
}

// Toggle character dropdown
function toggleCharacterDropdown(show = null) {
    const dropdown = document.getElementById('character-dropdown');
    if (show === null) {
        dropdown.classList.toggle('hidden');
    } else {
        dropdown.classList.toggle('hidden', !show);
    }
}

// Avatar selector click handler
document.getElementById('avatar-selector').addEventListener('click', () => {
    toggleCharacterDropdown();
});

// Click outside to close dropdown
document.addEventListener('click', (e) => {
    if (!e.target.closest('#avatar-selector') && !e.target.closest('#character-dropdown')) {
        toggleCharacterDropdown(false);
    }
});

// Textarea auto-resize and show/hide submit button
const textarea = document.getElementById('comment-textarea');
const submitBtn = document.getElementById('comment-submit');

textarea.addEventListener('input', function () {
    // Auto-resize
    this.style.height = 'auto';
    this.style.height = this.scrollHeight + 'px';

    // Show/hide submit button
    if (this.value.trim()) {
        submitBtn.classList.remove('hidden');
    } else {
        submitBtn.classList.add('hidden');
    }
});

// Enter key to submit (Shift+Enter for new line)
textarea.addEventListener('keydown', function (e) {
    if (e.key === 'Enter' && !e.shiftKey) {
        e.preventDefault();
        if (this.value.trim()) {
            document.getElementById('comment-form').dispatchEvent(new Event('submit'));
        }
    }
});

// File selection indicator
document.getElementById('comment-media').addEventListener('change', function () {
    const indicator = document.getElementById('files-indicator');
    if (this.files.length > 0) {
        indicator.textContent = `${this.files.length} file(s) selected`;
        indicator.classList.remove('hidden');
    } else {
        indicator.classList.add('hidden');
    }
});

// Form submission
document.getElementById('comment-form').addEventListener('submit', async function (e) {
    e.preventDefault();

    const formData = new FormData(this);

    try {
        const res = await fetch('/api/post/', {
            method: 'POST',
            body: formData
        });

        if (res.ok) {
            // Reset form
            textarea.value = '';
            textarea.style.height = 'auto';
            submitBtn.classList.add('hidden');
            document.getElementById('comment-media').value = '';
            document.getElementById('files-indicator').classList.add('hidden');

            // Reload page or refresh comments
            location.reload();
        } else {
            const error = await res.text();
            console.error('Failed to post comment:', error);
            alert('Failed to post comment');
        }
    } catch (err) {
        console.error('Error posting comment:', err);
        alert('Error posting comment');
    }
});

// Load characters on page load
loadCharacters();
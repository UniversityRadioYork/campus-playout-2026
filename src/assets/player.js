/**
 * @type HTMLAudioElement
 */
const player = document.getElementById('player');
/**
 * @type HTMLButtonElement
 */
const button = document.getElementById('toggle-button');

const streamUrl = document.currentScript.dataset.stream;

let isPaused = true;
let isLoading = true;

if (Hls.isSupported()) {
    const hls = new Hls();
    hls.loadSource(streamUrl);
    hls.attachMedia(player);

    hls.on(Hls.Events.ERROR, (_, data) => {
        if (data.fatal) {
            switch (data.type) {
                case Hls.ErrorTypes.MEDIA_ERROR:
                    console.error('attempting to recover from fatal media error');
                    hls.recoverMediaError();
                    break;
                case Hls.ErrorTypes.NETWORK_ERROR:
                    console.error('fatal network error occured');
                    break;
                default:
                    console.error('unrecoverable error occurred');
                    hls.destroy();
            }
        }
    });
} else if (player.canPlayType('application/vnd.apple.mpegurl')) {
    player.src = streamUrl;
} else {
    document.currentScript.outerHTML = "<h2>Your browser is not supported!</h2>";
    button.disabled = true;
}

player.addEventListener('play', eventWrapper(() => (isPaused = false)));
player.addEventListener('pause', eventWrapper(() => (isPaused = true)));
// safari iOS seems to send the onWaiting event when the stream is still playing, so we make sure we aren't been lied to
player.addEventListener('stalled', eventWrapper((e) => (isLoading = e.currentTarget.readyState < HTMLMediaElement.HAVE_FUTURE_DATA)));
player.addEventListener('playing', eventWrapper(() => (isLoading = false)));
player.addEventListener('loadedmetadata', eventWrapper(() => (isLoading = false)));
player.addEventListener('loadeddata', eventWrapper((e) => {
    if (e.currentTarget.readyState >= HTMLMediaElement.HAVE_FUTURE_DATA) {
        isLoading = false;
    }
}));

function updateButton() {
    button.disabled = isLoading;
    if (isLoading) {
        button.classList.remove('btn-primary');
        button.classList.add('btn-outline-primary');
        button.innerHTML = `
            <span class="spinner-border spinner-border-sm" aria-hidden="true"></span>
            <span class="visually-hidden" role="status">Loading...</span>
        `;
    } else {
        button.innerHTML = isPaused ? 'Start' : 'Stop';
        if (isPaused) {
            button.classList.remove('btn-primary');
            button.classList.add('btn-outline-primary');
        } else {
            button.classList.add('btn-primary');
            button.classList.remove('btn-outline-primary');
        }
    }
}

function eventWrapper(f) {
    return (e) => {
        f(e);
        updateButton();
    }
}

function togglePause() {
    if (isLoading) return;
    if (player.paused) {
        player.currentTime = player.duration - 1;
        player.play();
    } else {
        player.pause();
    }
}

button.addEventListener('click', function() {
    togglePause();
});

updateButton();

#!/usr/bin/env liquidsoap

settings.frame.audio.channels := 2
settings.frame.audio.samplerate := 48000

settings.server.socket := true

# bodge for docker :(
settings.init.allow_root := true

enable_autocue_metadata()
settings.autocue.preferred.set("internal")
settings.autocue.internal.metadata_override.set([""])
settings.autocue.internal.lufs_target.set(-23.0)
settings.autocue.amplify_behavior.set("override")

settings.lufs.track_gain_target := -18.

enable_lufs_track_gain_metadata()

def get_env(key) =
  value = environment.get(key)
  if value != "" then
    value
  else
    log.critical("Must set the `#{key}` environment variable")
    thread.delay(1.0)
    shutdown(code=123)
    ""
  end
end

api_token = get_env("API_TOKEN")
srt_host = get_env("SRT_HOST")
srt_port = int_of_string(get_env("SRT_PORT"))
stream_id = get_env("SRT_STREAM_ID")
playlist_file = get_env("PLAYLIST_FILE")
jingles_file = get_env("JINGLES_FILE")
morning_jingles_file = get_env("MORNING_JINGLES_FILE")
afternoon_jingles_file = get_env("AFTERNOON_JINGLES_FILE")
evening_jingles_file = get_env("EVENING_JINGLES_FILE")
unix_socket_path = get_env("UNIX_SOCKET_PATH")
srt_username = get_env("SRT_USERNAME")
srt_password = get_env("SRT_PASSWORD")

settings.server.socket.path := unix_socket_path

def get_trackid(meta) =
  trackid = list.assoc(default="", "trackid", meta)
  if trackid == "" then
    null
  else
    int_of_string(trackid)
  end
end

def was_request(meta) =
  bool_of_string(list.assoc(default="false", "is_request", meta))
end

def new_metadata(meta) =
  ignore(http.post(
    data = json.stringify({
      trackid = get_trackid(meta),
      was_request = was_request(meta),
    }),
    headers = [
      ("Content-Type", "application/json; charset=UTF-8"),
      ("Authorization", "Bearer #{api_token}")
    ],
    "http://localhost:3000/__internal/metadata"
  ))
end

# yes i know it's in the URL
# give me a better way and i'll change it
jingles = playlist("http://localhost:3000/__internal/jingles.m3u8?api_key=#{api_token}", reload_mode="seconds", reload=60)

tracks = playlist(playlist_file, reload_mode="watch")

track_requests = request.queue(id="track_requests")

tracks = fallback([track_requests, tracks])

def skip_track(_) =
  tracks.skip()
  "Done\n"
end
server.register(
  namespace="playout",
  "skip",
  skip_track
)

playout = random(weights=[1, 3], [jingles, tracks])
playout = crossfade(playout)
playout = mksafe(playout)

playout.on_metadata(synchronous=false, new_metadata)

enc = %ffmpeg(
  format = "mpegts",
  interleaved = false,
  %audio(
    codec = "aac",
    channels = 2,
    ar = 48000,
    b = "192k",
    profile = "aac_low"
  )
)

output.srt(
  host=srt_host,
  port=srt_port,
  mode="caller",
  streamid="publish:#{stream_id}:#{srt_username}:#{srt_password}",
  enc,
  playout
)

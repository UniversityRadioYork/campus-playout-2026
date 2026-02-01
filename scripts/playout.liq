#!/usr/bin/env liquidsoap

settings.frame.audio.channels := 2
settings.frame.audio.samplerate := 48000

settings.server.telnet := true

# bodge for docker :(
settings.init.allow_root := true

enable_autocue_metadata()
settings.autocue.preferred.set("internal")
settings.autocue.internal.metadata_override.set([""])
settings.autocue.internal.lufs_target.set(-23.0)
settings.autocue.amplify_behavior.set("override")

def get_env(key) =
  value = environment.get(key)
  if value != "" then
    value
  else
    log.critical("Must set the `#{key}` environment variable")
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
srt_username = get_env("SRT_USERNAME")
srt_password = get_env("SRT_PASSWORD")

def get_trackid(meta) =
  trackid = list.assoc(default="", "trackid", meta)
  if trackid == "" then
    null()
  else
    int_of_string(trackid)
  end
end

def new_metadata(meta) =
  ignore(http.post(
    data = json.stringify({
      trackid = get_trackid(meta),
    }),
    headers = [
      ("Content-Type", "application/json; charset=UTF-8"),
      ("Authorization", "Bearer #{api_token}")
    ],
    "http://localhost:3000/__internal/metadata"
  ))
end

jingles = playlist(jingles_file)
tracks = playlist(playlist_file, reload_mode="watch")

playout = random(weights=[1, 5], [jingles, tracks])

playout = crossfade(playout)

playout = mksafe(playout)

playout.on_metadata(new_metadata)

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

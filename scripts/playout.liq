#!/usr/bin/env liquidsoap

settings.frame.audio.channels := 2
settings.frame.audio.samplerate := 48000

settings.server.telnet := true

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

jingles = playlist("test/jingles.txt")
tracks = playlist("test/playlist.txt", reload_mode="watch")

playout = random(weights=[1, 5], [jingles, tracks])

playout = crossfade(playout)

playout = mksafe(playout)

playout.on_metadata(new_metadata)

output(playout)

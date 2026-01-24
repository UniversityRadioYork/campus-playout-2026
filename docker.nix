{ dockerTools
, buildEnv
, writeShellScriptBin
, tini
, campus-playout
, campus-playout-streamer
}:

dockerTools.streamLayeredImage {
  name = "campus-playout-2026";
  tag = "latest";
  contents = [
    tini
    campus-playout
    campus-playout-streamer
    (writeShellScriptBin "entrypoint" ''
      set -eux -o pipefail
      /bin/campus-playout-streamer &
      /bin/campus-playout
    '')
  ];

  config = {
    Cmd = [ "/bin/tini" "/bin/entrypoint" ];
    WorkingDir = "/data";
  };
}

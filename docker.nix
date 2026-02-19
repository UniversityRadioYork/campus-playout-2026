{ dockerTools
, buildEnv
, writeShellScriptBin
, runCommand
, tini
, campus-playout
, campus-playout-streamer
}:
let
  tmp = runCommand "tmp" { } ''
    mkdir $out
    mkdir -m 1777 $out/tmp
  '';
in

dockerTools.streamLayeredImage {
  name = "campus-playout-2026";
  tag = "latest";
  contents = [
    tini
    campus-playout
    campus-playout-streamer
    (dockerTools.caCertificates)
    tmp
    (writeShellScriptBin "entrypoint" ''
      set -eux -o pipefail
      /bin/campus-playout-streamer &
      /bin/campus-playout-2026
    '')
  ];

  config = {
    Cmd = [ "/bin/tini" "/bin/entrypoint" ];
    WorkingDir = "/data";
    Env = [
      "DATABASE_URL=\"sqlite:/data/database.db\""
      "JINGLES_FILE=\"/data/jingles.txt\""
      "MORNING_JINGLES_FILE=\"/data/morning-jingles.txt\""
      "AFTERNOON_JINGLES_FILE=\"/data/afternoon-jingles.txt\""
      "EVENING_JINGLES_FILE=\"/data/evening-jingles.txt\""
      "PLAYLIST_FILE=\"/data/playlist-gen.txt\""
      "UNIX_SOCKET_PATH=\"/data/liq.sock\""
    ];
  };
}

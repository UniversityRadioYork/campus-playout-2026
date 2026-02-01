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
  };
}

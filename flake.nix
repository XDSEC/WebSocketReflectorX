{
  description = "WebSocketReflectorX CLI flake";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      systems = [
        "x86_64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forAllSystems = nixpkgs.lib.genAttrs systems;
      version = "0.5.16";
      assets = {
        x86_64-linux = {
          url = "https://github.com/XDSEC/WebSocketReflectorX/releases/download/${version}/wsrx-cli-${version}-linux-gnu-x86_64.tar.gz";
          sha256 = "0fixr2hhj707yk5w7l505gdmhdb45pyhlsb45lps3794qihvqs4f";
        };
        x86_64-darwin = {
          url = "https://github.com/XDSEC/WebSocketReflectorX/releases/download/${version}/wsrx-cli-${version}-macos-x86_64.zip";
          sha256 = "1kfhgyna7i7259mapmz540favvdp1a2ac90ka08a1lf7blzgksk1";
        };
        aarch64-darwin = {
          url = "https://github.com/XDSEC/WebSocketReflectorX/releases/download/${version}/wsrx-cli-${version}-macos-aarch64.zip";
          sha256 = "0a3ylyhy2c0b1212dlw1q3z2r46kafkb22d9mmps381psvgbn038";
        };
      };
    in
    {
      packages = forAllSystems (system:
        let
          pkgs = import nixpkgs { inherit system; };
          asset = assets.${system};
          pkg = pkgs.stdenvNoCC.mkDerivation {
            pname = "wsrx";
            inherit version;
            src = pkgs.fetchurl {
              inherit (asset) url sha256;
            };
            nativeBuildInputs = [ pkgs.unzip ];
            dontUnpack = true;
            installPhase = ''
              runHook preInstall

              case "$src" in
                *.tar.gz) tar -xzf "$src" ;;
                *.zip) unzip -q "$src" ;;
                *) echo "Unsupported archive: $src" >&2; exit 1 ;;
              esac

              bin_path="$(find . -type f -name wsrx | head -n1)"
              install -Dm755 "$bin_path" "$out/bin/wsrx"

              runHook postInstall
            '';
            meta = with pkgs.lib; {
              description = "Controlled TCP-over-WebSocket forwarding tunnel";
              homepage = "https://github.com/XDSEC/WebSocketReflectorX";
              license = licenses.mit;
              mainProgram = "wsrx";
              platforms = builtins.attrNames assets;
            };
          };
        in
        {
          default = pkg;
          wsrx = pkg;
        });

      apps = forAllSystems (system:
        let
          pkg = self.packages.${system}.default;
        in
        {
          default = {
            type = "app";
            program = "${pkg}/bin/wsrx";
          };
          wsrx = {
            type = "app";
            program = "${pkg}/bin/wsrx";
          };
        });
    };
}

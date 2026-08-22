{
  description = "WebSocketReflectorX";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      inherit (nixpkgs) lib;

      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      forAllSystems = lib.genAttrs systems;

      workspace = builtins.fromTOML (builtins.readFile ./Cargo.toml);
      version = workspace.workspace.package.version;

      mkPkgs = system:
        import nixpkgs {
          inherit system;
        };
    in
    {
      packages = forAllSystems (system:
        let
          pkgs = mkPkgs system;

          inherit (pkgs) stdenv;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          commonNativeBuildInputs = with pkgs; [
            pkg-config
          ];

          commonBuildInputs = lib.optionals stdenv.isDarwin (with pkgs.darwin.apple_sdk.frameworks; [
            AppKit
            CoreFoundation
            CoreGraphics
            CoreServices
            Foundation
            Security
          ]);

          linuxDesktopLibraries = with pkgs; [
            fontconfig
            freetype
            libGL
            libxkbcommon
            libx11
            libxcb
            libxcb-cursor
            libxcb-image
            libxcb-keysyms
            libxcb-render-util
            libxcb-util
            libxcb-wm
            libxcursor
            libxi
            libxkbfile
            libxrandr
            wayland
            vulkan-loader
          ];

          desktopBuildInputs =
            commonBuildInputs
            ++ lib.optionals stdenv.isLinux linuxDesktopLibraries;

          commonArgs = {
            inherit version cargoLock;

            src = lib.cleanSource ./.;

            nativeBuildInputs = commonNativeBuildInputs;

            buildInputs = commonBuildInputs;

            WSRX_GIT_VERSION =
              self.shortRev or self.dirtyShortRev or "unknown";

            meta = with lib; {
              homepage = "https://github.com/XDSEC/WebSocketReflectorX";
              license = licenses.mit;
              maintainers = [ ];
            };
          };

          wsrx = pkgs.rustPlatform.buildRustPackage (commonArgs // {
            pname = "wsrx";

            cargoBuildFlags = [
              "-p"
              "wsrx"
            ];

            cargoTestFlags = [
              "-p"
              "wsrx"
            ];

            meta = commonArgs.meta // {
              description = "Controlled TCP-over-WebSocket forwarding tunnel";
              mainProgram = "wsrx";
            };
          });

          wsrx-desktop = pkgs.rustPlatform.buildRustPackage (commonArgs // {
            pname = "wsrx-desktop";

            nativeBuildInputs =
              commonNativeBuildInputs
              ++ lib.optionals stdenv.isLinux (with pkgs; [
                curl
                makeWrapper
                python3
              ]);

            buildInputs = desktopBuildInputs;

            cargoBuildFlags = [
              "-p"
              "wsrx-desktop"
            ];

            cargoTestFlags = [
              "-p"
              "wsrx-desktop"
            ];

            postInstall = lib.optionalString stdenv.isLinux ''
              install -Dm644 freedesktop/wsrx-desktop.desktop \
                "$out/share/applications/wsrx-desktop.desktop"
              install -Dm644 freedesktop/wsrx-desktop.svg \
                "$out/share/icons/hicolor/scalable/apps/wsrx-desktop.svg"

              wrapProgram "$out/bin/wsrx-desktop" \
                --prefix LD_LIBRARY_PATH : ${lib.makeLibraryPath linuxDesktopLibraries}
            '';

            meta = commonArgs.meta // {
              description = "Desktop interface for WebSocketReflectorX";
              mainProgram = "wsrx-desktop";
            };
          });
        in
        {
          default = wsrx;
          inherit wsrx wsrx-desktop;
        });

      apps = forAllSystems (system:
        let
          mkApp = packageName: {
            type = "app";
            program =
              "${self.packages.${system}.${packageName}}/bin/${packageName}";
          };
        in
        {
          default = mkApp "wsrx";
          wsrx = mkApp "wsrx";
          wsrx-desktop = mkApp "wsrx-desktop";
        });

      devShells = forAllSystems (system:
        let
          pkgs = mkPkgs system;
        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              cargo
              rustc
              rustfmt
              clippy
              pkg-config
            ] ++ lib.optionals pkgs.stdenv.isLinux [
              libGL
              libxkbcommon
              wayland
              libx11
              libxcb
              libxcb-cursor
            ];
          };
        });
    };
}

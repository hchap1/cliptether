{ pkgs, lib, ... }:

{
  languages.rust.enable = true;
  packages = [ pkgs.pkg-config pkgs.wayland pkgs.dbus ];

  enterShell = ''
    export LD_LIBRARY_PATH="${lib.makeLibraryPath [ pkgs.dbus ]}:$LD_LIBRARY_PATH"
  '';
}

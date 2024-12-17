{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    fenix,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        system = system;
        config.allowUnfree = true;
        config.android_sdk.accept_license = true;
      };

      android_composition =
        (pkgs.androidenv.composeAndroidPackages {
          platformVersions = ["34"];
          ndkVersions = ["26.3.11579264"];
	  buildToolsVersions = [ "34.0.0" ];
          includeNDK = true;
          useGoogleAPIs = false;
          useGoogleTVAddOns = false;
          includeEmulator = true;
          includeSystemImages = false;
          includeSources = true;
        });
      
      packages = with pkgs; [
        curl
        wget
        pkg-config
	trunk

        nodejs_20
	tailwindcss

        (with fenix.packages.${system};
          combine [
            complete.rustc
            complete.cargo
            complete.clippy
            targets.aarch64-linux-android.latest.rust-std
            targets.armv7-linux-androideabi.latest.rust-std
            targets.i686-linux-android.latest.rust-std
            targets.x86_64-linux-android.latest.rust-std
	    targets.wasm32-unknown-unknown.latest.rust-std
          ])
        rust-analyzer

        android_composition.androidsdk
        jdk
      ];

      libraries = with pkgs; [
        gtk3
        libsoup_3
        webkitgtk_4_1
        cairo
        gdk-pixbuf
        glib
        dbus
        openssl_3
        librsvg
      ];
    in {
      devShell = pkgs.mkShell rec {
        buildInputs = packages ++ libraries;

	shellHook = ''
	  fish
	'';

        NIX_LD = "${pkgs.stdenv.cc.libc}/lib/ld-linux-x86-64.so.2";
        ANDROID_HOME = "${android_composition.androidsdk}/libexec/android-sdk";
        NDK_HOME = "${android_composition.androidsdk}/libexec/android-sdk/ndk/${builtins.head (pkgs.lib.lists.reverseList (builtins.split "-" "${android_composition.ndk-bundle}"))}";
        ANDROID_SDK_ROOT = "${android_composition.androidsdk}/libexec/android-sdk";
        ANDROID_NDK_ROOT = "${android_composition.androidsdk}/libexec/android-sdk/ndk-bundle";
      };
    });
}

{ inputs, pkgs, ... }:

let
    toolchain = inputs.fenix.packages.${pkgs.system}.fromToolchainFile {
        file = ../../rust-toolchain.toml;
        sha256 = "sha256-uKJ5ShQ7u7lK1ygIwqtsOnxRnoQkJVsSvxsehf1Ilp8=";
    };

    naersk' = pkgs.callPackage inputs.naersk {
        cargo = toolchain;
        rustc = toolchain;
    };
in

naersk'.buildPackage { src = ../..; }

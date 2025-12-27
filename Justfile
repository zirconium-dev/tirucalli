binary_name := "redirect-niri"

setup-binds $profile="./binds.yaml":
    #!/usr/bin/env bash
    busctl call org.shadowblip.InputPlumber \
      /org/shadowblip/InputPlumber/CompositeDevice0 \
      org.shadowblip.Input.CompositeDevice \
      LoadProfilePath "s" "$(realpath "${profile}")"

run:
    cargo build --release
    ./target/release/redirect-niri

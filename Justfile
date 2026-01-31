binary_name := "redirect-niri"

plumber-less-annoying:
    inputplumber devices manage-all --enable

setup-binds $profile="./binds.yaml":
    #!/usr/bin/env bash
    busctl call org.shadowblip.InputPlumber \
      /org/shadowblip/InputPlumber/CompositeDevice0 \
      org.shadowblip.Input.CompositeDevice \
      LoadProfilePath "s" "$(realpath "${profile}")"

run:
    cargo build
    ./target/release/redirect-niri

binary_name := "redirect-niri"

privileged-run $binary_name=binary_name:
    cargo build
    sudo ./target/debug/{{binary_name}}

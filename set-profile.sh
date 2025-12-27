#!/usr/bin/env bash

set -x

busctl set-property org.shadowblip.InputPlumber \
  /org/shadowblip/InputPlumber/CompositeDevice0 \
  org.shadowblip.Input.CompositeDevice \
  InterceptMode u 0

# profile="/usr/share/inputplumber/profiles/mouse_keyboard_wasd.yaml"
profile="./binds.yaml"

busctl call org.shadowblip.InputPlumber \
  /org/shadowblip/InputPlumber/CompositeDevice0 \
  org.shadowblip.Input.CompositeDevice \
  LoadProfilePath "s" $(realpath $profile)

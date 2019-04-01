#!/bin/bash

set -euo pipefail

bin_dir="${HOME}/habitat/target/debug"

sudo docker run \
     --network="${NETWORK}" \
     --network-alias=dev.habitat.dev \
     --mount type=bind,source=/hab/cache,target=/hab/cache \
     --mount type=bind,source=/hab/pkgs,target=/hab/pkgs \
     --mount type=bind,source="${bin_dir}/hab",target=/bin/hab \
     --mount type=bind,source="${bin_dir}/hab-sup",target=/bin/hab-sup \
     --mount type=bind,source="${bin_dir}/hab-launch",target=/bin/hab-launch \
     --mount type=bind,source="$(pwd)/data/dev/hab-sup-default",target=/hab/sup/default \
     --env="HAB_SUP_BINARY=/bin/hab-sup" \
     --env="HAB_LAUNCH_BINARY=/bin/hab-launch" \
     --env="HAB_LAUNCH_NO_SUP_VERSION_CHECK=1" \
     --env="RUST_LOG=debug,rustc_metadata=error,cargo=error,jobserver=error,rustc_trans=error,rustc_driver=error,rustc_mir=error,rustc=error,tokio_core=info" \
     --entrypoint="/bin/hab" \
     --name=habitat_dev \
     --rm \
     --interactive \
     --tty \
     terrarium-dev-base sup run ${@}

#!/usr/bin/env bash
# Regenerate backend/THIRD-PARTY-NOTICES.txt: the licence of every crate the server and daemon
# ship (cargo-about, configured by backend/about.toml and rendered through backend/about.hbs),
# followed by the licences of the third-party data files in backend/assets/.
#
# The daemon compiles the file in (`scanopy-daemon licenses`), the Docker images and the MSI
# carry it, and the UI build puts it at the top of /third-party-licenses.txt (ui/vite.config.ts).
#
# Invoked by `make third-party-notices` and by the release workflow
# (.github/workflows/release.yml, test-and-update-fixtures job), which commits the result to
# the release branch before any artifact is built.
#
# Fails when a crate's licence is not in about.toml's `accepted` list.
#
# Requires: cargo-about 0.9.3 (`cargo install cargo-about --version 0.9.3 --locked --features cli`).

set -euo pipefail
cd "$(dirname "$0")/.."

command -v cargo-about >/dev/null || {
    echo "ERROR: cargo-about is required: cargo install cargo-about --version 0.9.3 --locked --features cli" >&2
    exit 1
}

out=THIRD-PARTY-NOTICES.txt
tmp=$(mktemp)
trap 'rm -f "$tmp"' EXIT

cargo about generate --locked --fail about.hbs -o "$tmp"

# <licence file> <what it covers>
append_data_licence() {
    {
        printf '\n--------------------------------------------------------------------------------\n'
        printf '%s\n' "$2"
        printf -- '--------------------------------------------------------------------------------\n\n'
        cat "$1"
    } >> "$tmp"
}

{
    printf '\n================================================================================\n'
    printf 'BUNDLED DATA\n'
    printf '================================================================================\n'
} >> "$tmp"

append_data_licence assets/recog/LICENSE \
    "Rapid7 Recog fingerprints (https://github.com/rapid7/recog), compiled into the server and daemon"
append_data_licence assets/disposable-email-domains.LICENSE \
    "disposable-email-domains list, compiled into the server"
append_data_licence assets/dashboard-icons/LICENSE \
    "Dashboard Icons (https://github.com/homarr-labs/dashboard-icons), logos shown in the UI"
append_data_licence assets/libvirt/LICENSE \
    "libvirt logo (https://libvirt.org/logos/), shown in the UI"

mv "$tmp" "$out"
trap - EXIT
echo "wrote backend/$out"

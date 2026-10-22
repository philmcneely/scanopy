#!/usr/bin/env bash
# Fail unless every image a release's notes use is on scanopy/website main.
#
# A note's images live at scanopy.net/changelog/vX.Y.Z/<file>, which the website
# serves from static/changelog/vX.Y.Z/<file>. The release email and the in-app modal
# load them from there as soon as servers run the release, well before
# promote_to_latest.yml publishes the changelog page, so publish_release.yml runs
# this before it creates the release. The backend's release notes tests already
# reject an image hosted anywhere else.
#
# Usage: check-release-note-images.sh <version>. Needs GH_TOKEN with read access to
# the website repo. WEBSITE_REPO overrides the repo, for testing.
set -euo pipefail

VERSION="$1"
FILE="backend/release-notes/v$VERSION.md"
REPO="${WEBSITE_REPO:-scanopy/website}"

if [ -z "${GH_TOKEN:-}" ]; then
    echo "::error::GH_TOKEN is not set, so the release note images on $REPO can't be checked"
    exit 1
fi

# Every URL under the release's folder, whether an inline image, a reference
# definition or a link.
urls=()
while IFS= read -r url; do urls+=("$url"); done < <(grep -oE "https://scanopy\.net/changelog/v${VERSION//./\\.}/[^][)[:space:]\"'<>]+" "$FILE" | sort -u)
if [ ${#urls[@]} -eq 0 ]; then
    echo "No images in $FILE"
    exit 0
fi

missing=0
for url in "${urls[@]}"; do
    path="static/${url#https://scanopy.net/}"
    path="${path%%[?#]*}"
    if output="$(gh api "repos/$REPO/contents/$path?ref=main" --silent 2>&1)"; then
        echo "$url: on $REPO main"
    elif grep -q "HTTP 404" <<<"$output"; then
        echo "::error file=$FILE::$url is not on $REPO main ($path). Commit it there, then re-run Prepare Release for $VERSION: the release email and modal load it."
        missing=1
    else
        echo "::error::Could not check $path on $REPO: $output"
        exit 1
    fi
done
exit "$missing"

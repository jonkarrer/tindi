publish-changelog version:
    git cliff -u --tag {{version}} --prepend CHANGELOG.md

test function:
    cargo test {{function}}
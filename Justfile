features := "samd21g18a samd21e18a samd21j18a samd51j19a samd51j20a samd51g19a"

release version:
    #!/usr/bin/env sh
    git diff HEAD --exit-code --name-only
    if [ -z $(git ls-files --exclude-standard --others) ]; then
        echo "Cannot make release with a dirty working directory."
    else
        just build-docs
        cargo readme > README.md
        cargo bump -g {{ version }}

    fi

build-docs:
    #!/usr/bin/env sh
    cargo clean --doc
    for feature in {{features}}; do
        cargo doc --no-deps --features $feature
        cp -nR target/doc/* docs/
        mv docs/samd_dma docs/$feature
    done
    cargo readme > docs/index.md
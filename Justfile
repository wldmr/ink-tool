diff := "difft"
cargo_profile := "dev"

# Test rust code
[group('tests')]
test *args:
    cargo test {{args}}

# Test ink code
[group('tests')]
test-inklecate pattern="*.test.ink":
    find . -iname "{{pattern}}" | xargs cargo run -- test

# Test everything
[group('tests')]
test-all:
    just test --workspace
    just test --workspace --examples
    just test --workspace --doc
    just test-inklecate

diff target="examples/the_intercept.ink":
    #!/usr/bin/env sh
    result={{target}}
    result=${result%.ink}.fmt.ink
    cat {{target}} | cargo run - > $result
    echo -e "\n\n"
    {{diff}} "{{target}}" "$result"

sample target="examples/input" output="examples/output":
    #!/usr/bin/env sh
    samplyfile={{target}}.sample.`git log -1 --format="%cI.%h"`.cargo_{{cargo_profile}}.geckoprofile.json
    samply record --save-only -o $samplyfile -- cargo run --profile {{cargo_profile}} -- fmt {{target}} -o {{output}}
    # format the file
    jq . $samplyfile | sponge $samplyfile
    echo $samplyfile

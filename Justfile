diff := "difft"
cargo_profile := "dev"

test filter="":
    cargo test {{ filter }}

diff target="examples/the_intercept.ink":
    #!/usr/bin/env sh
    result={{target}}
    result=${result%.ink}.fmt.ink
    cat {{target}} | cargo run - > $result
    echo -e "\n\n"
    {{diff}} "{{target}}" "$result"

sample target="examples/the_intercept.ink":
    #!/usr/bin/env sh
    samplyfile={{target}}.`git log -1 --format="%cI.%h"`.cargo_{{cargo_profile}}.geckoprofile.json
    samply record --save-only -o $samplyfile -- cargo run --profile {{cargo_profile}} -- {{target}}
    # format the file
    jq . $samplyfile | sponge $samplyfile
    echo $samplyfile

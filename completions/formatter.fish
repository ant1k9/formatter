complete -f -c formatter \
    -l help \
    -d "show usage for command"

complete -f -c formatter \
    -l prefix \
    -n "not __fish_seen_subcommand_from --prefix" \
    -d "prefix for every line"

complete -f -c formatter \
    -l suffix \
    -n "not __fish_seen_subcommand_from --suffix" \
    -d "suffix for every line"

complete -f -c formatter \
    -l header \
    -n "not __fish_seen_subcommand_from --header" \
    -d "header for the whole output"

complete -f -c formatter \
    -l footer \
    -n "not __fish_seen_subcommand_from --footer" \
    -d "footer for the whole output"

complete -f -c formatter \
    -l line-breaker \
    -n "not __fish_seen_subcommand_from --line-breaker" \
    -d "line breaker between output lines"

complete -f -c formatter \
    -l width \
    -n "not __fish_seen_subcommand_from --width" \
    -d "fix width for every line"

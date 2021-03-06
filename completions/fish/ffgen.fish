complete -c ffgen -n "__fish_use_subcommand" -s h -l help -d 'Print help information'
complete -c ffgen -n "__fish_use_subcommand" -s V -l version -d 'Print version information'
complete -c ffgen -n "__fish_use_subcommand" -f -a "group" -d 'Generate feature group data'
complete -c ffgen -n "__fish_use_subcommand" -f -a "label" -d 'Generate feature label data'
complete -c ffgen -n "__fish_use_subcommand" -f -a "schema" -d 'Generate feature store schema'
complete -c ffgen -n "__fish_use_subcommand" -f -a "list" -d 'List available groups'
complete -c ffgen -n "__fish_use_subcommand" -f -a "completion" -d 'Output shell completion code'
complete -c ffgen -n "__fish_use_subcommand" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c ffgen -n "__fish_seen_subcommand_from group" -s I -l id-range -d 'ID range (example: 1000..2000)' -r
complete -c ffgen -n "__fish_seen_subcommand_from group" -s s -l seed -d 'Seed for the random generator' -r
complete -c ffgen -n "__fish_seen_subcommand_from group" -s r -l recipe -d 'Recipe file path' -r
complete -c ffgen -n "__fish_seen_subcommand_from group" -s f -l format -d 'Data format' -r -f -a "{csv	,json	,yaml	}"
complete -c ffgen -n "__fish_seen_subcommand_from group" -s h -l help -d 'Print help information'
complete -c ffgen -n "__fish_seen_subcommand_from label" -s I -l id-range -d 'ID range (example: 1000..2000)' -r
complete -c ffgen -n "__fish_seen_subcommand_from label" -s T -l time-range -d 'Label time range (example: 2022-01-01..2022-01-31)' -r
complete -c ffgen -n "__fish_seen_subcommand_from label" -s l -l limit -d 'Max entries to generate' -r
complete -c ffgen -n "__fish_seen_subcommand_from label" -s s -l seed -d 'Seed for the random generator' -r
complete -c ffgen -n "__fish_seen_subcommand_from label" -s r -l recipe -d 'Recipe file path' -r
complete -c ffgen -n "__fish_seen_subcommand_from label" -s f -l format -d 'Data format' -r -f -a "{csv	,json	,yaml	}"
complete -c ffgen -n "__fish_seen_subcommand_from label" -s h -l help -d 'Print help information'
complete -c ffgen -n "__fish_seen_subcommand_from schema" -s r -l recipe -d 'Recipe file path' -r
complete -c ffgen -n "__fish_seen_subcommand_from schema" -l format -d 'Schema format' -r -f -a "{yaml	,json	,toml	}"
complete -c ffgen -n "__fish_seen_subcommand_from schema" -s h -l help -d 'Print help information'
complete -c ffgen -n "__fish_seen_subcommand_from list" -s r -l recipe -d 'Recipe file path' -r
complete -c ffgen -n "__fish_seen_subcommand_from list" -s h -l help -d 'Print help information'
complete -c ffgen -n "__fish_seen_subcommand_from completion" -s h -l help -d 'Print help information'

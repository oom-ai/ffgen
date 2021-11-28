complete -c ffgen -n "__fish_use_subcommand" -s h -l help -d 'Print help information'
complete -c ffgen -n "__fish_use_subcommand" -s V -l version -d 'Print version information'
complete -c ffgen -n "__fish_use_subcommand" -f -a "group" -d 'Generate feature group data'
complete -c ffgen -n "__fish_use_subcommand" -f -a "label" -d 'Generate feature label data'
complete -c ffgen -n "__fish_use_subcommand" -f -a "schema" -d 'Generate feature store schema'
complete -c ffgen -n "__fish_use_subcommand" -f -a "list" -d 'List available resources'
complete -c ffgen -n "__fish_use_subcommand" -f -a "completion" -d 'Output shell completion code'
complete -c ffgen -n "__fish_use_subcommand" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c ffgen -n "__fish_seen_subcommand_from group" -s I -l id-range -d 'ID range' -r
complete -c ffgen -n "__fish_seen_subcommand_from group" -l seed -d 'Seed for the random generator' -r
complete -c ffgen -n "__fish_seen_subcommand_from group" -s s -l schema -d 'Schema file for ffgen' -r
complete -c ffgen -n "__fish_seen_subcommand_from group" -s f -l format -d 'Data format' -r -f -a "{csv	,json	}"
complete -c ffgen -n "__fish_seen_subcommand_from group" -s h -l help -d 'Print help information'
complete -c ffgen -n "__fish_seen_subcommand_from label" -s I -l id-range -d 'ID range' -r
complete -c ffgen -n "__fish_seen_subcommand_from label" -s T -l time-range -d 'Label time range' -r
complete -c ffgen -n "__fish_seen_subcommand_from label" -l limit -d 'Max entries to generate' -r
complete -c ffgen -n "__fish_seen_subcommand_from label" -l seed -d 'Seed for the random generator' -r
complete -c ffgen -n "__fish_seen_subcommand_from label" -s s -l schema -d 'Schema file for ffgen' -r
complete -c ffgen -n "__fish_seen_subcommand_from label" -s f -l format -d 'Data format' -r -f -a "{csv	,json	}"
complete -c ffgen -n "__fish_seen_subcommand_from label" -s h -l help -d 'Print help information'
complete -c ffgen -n "__fish_seen_subcommand_from schema" -s s -l schema -d 'Schema file for ffgen' -r
complete -c ffgen -n "__fish_seen_subcommand_from schema" -s h -l help -d 'Print help information'
complete -c ffgen -n "__fish_seen_subcommand_from list" -s s -l schema -d 'Schema file for ffgen' -r
complete -c ffgen -n "__fish_seen_subcommand_from list" -s h -l help -d 'Print help information'
complete -c ffgen -n "__fish_seen_subcommand_from completion" -s h -l help -d 'Print help information'
complete -c ffgen -n "__fish_seen_subcommand_from help" -s h -l help -d 'Print help information'

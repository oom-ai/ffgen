complete -c ffgen -n "__fish_use_subcommand" -s h -l help -d 'Print help information'
complete -c ffgen -n "__fish_use_subcommand" -s V -l version -d 'Print version information'
complete -c ffgen -n "__fish_use_subcommand" -f -a "group" -d 'Generate feature group data'
complete -c ffgen -n "__fish_use_subcommand" -f -a "label" -d 'Generate feature label data'
complete -c ffgen -n "__fish_use_subcommand" -f -a "schema" -d 'Generate oomstore schema'
complete -c ffgen -n "__fish_use_subcommand" -f -a "list"
complete -c ffgen -n "__fish_use_subcommand" -f -a "completion" -d 'Generate shell completion file'
complete -c ffgen -n "__fish_use_subcommand" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c ffgen -n "__fish_seen_subcommand_from group" -s I -l id-range -d 'ID range' -r
complete -c ffgen -n "__fish_seen_subcommand_from group" -l seed -d 'Seed for the random generator' -r
complete -c ffgen -n "__fish_seen_subcommand_from group" -s s -l schema -d 'Schema file' -r
complete -c ffgen -n "__fish_seen_subcommand_from group" -s h -l help -d 'Print help information'
complete -c ffgen -n "__fish_seen_subcommand_from label" -s I -l id-range -d 'ID range' -r
complete -c ffgen -n "__fish_seen_subcommand_from label" -s T -l time-range -d 'Label time range' -r
complete -c ffgen -n "__fish_seen_subcommand_from label" -l limit -d 'Max entries to generate' -r
complete -c ffgen -n "__fish_seen_subcommand_from label" -l seed -d 'Seed for the random generator' -r
complete -c ffgen -n "__fish_seen_subcommand_from label" -s s -l schema -d 'Schema file' -r
complete -c ffgen -n "__fish_seen_subcommand_from label" -s h -l help -d 'Print help information'
complete -c ffgen -n "__fish_seen_subcommand_from schema" -s s -l schema -d 'Schema file' -r
complete -c ffgen -n "__fish_seen_subcommand_from schema" -s h -l help -d 'Print help information'
complete -c ffgen -n "__fish_seen_subcommand_from list" -s s -l schema -d 'Schema file' -r
complete -c ffgen -n "__fish_seen_subcommand_from list" -s h -l help -d 'Print help information'
complete -c ffgen -n "__fish_seen_subcommand_from completion" -s h -l help -d 'Print help information'
complete -c ffgen -n "__fish_seen_subcommand_from help" -s h -l help -d 'Print help information'

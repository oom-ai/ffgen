complete -c ffgen -n "__fish_use_subcommand" -l seed -r
complete -c ffgen -n "__fish_use_subcommand" -s h -l help -d 'Print help information'
complete -c ffgen -n "__fish_use_subcommand" -s V -l version -d 'Print version information'
complete -c ffgen -n "__fish_use_subcommand" -f -a "group" -d 'Generate feature group data'
complete -c ffgen -n "__fish_use_subcommand" -f -a "label" -d 'Generate feature label data'
complete -c ffgen -n "__fish_use_subcommand" -f -a "completion" -d 'Generate shell completion file'
complete -c ffgen -n "__fish_use_subcommand" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c ffgen -n "__fish_seen_subcommand_from group" -s I -l id-range -d 'ID range' -r
complete -c ffgen -n "__fish_seen_subcommand_from group" -l seed -r
complete -c ffgen -n "__fish_seen_subcommand_from group" -s h -l help -d 'Print help information'
complete -c ffgen -n "__fish_seen_subcommand_from label" -s I -l id-range -d 'Label id range' -r
complete -c ffgen -n "__fish_seen_subcommand_from label" -s T -l time-range -d 'Label time range' -r
complete -c ffgen -n "__fish_seen_subcommand_from label" -s l -l limit -d 'Max entries to generate' -r
complete -c ffgen -n "__fish_seen_subcommand_from label" -l seed -r
complete -c ffgen -n "__fish_seen_subcommand_from label" -s h -l help -d 'Print help information'
complete -c ffgen -n "__fish_seen_subcommand_from completion" -l seed -r
complete -c ffgen -n "__fish_seen_subcommand_from completion" -s h -l help -d 'Print help information'
complete -c ffgen -n "__fish_seen_subcommand_from help" -l seed -r
complete -c ffgen -n "__fish_seen_subcommand_from help" -s h -l help -d 'Print help information'

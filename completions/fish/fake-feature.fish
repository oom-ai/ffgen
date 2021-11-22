complete -c fake-feature -n "__fish_use_subcommand" -s h -l help -d 'Print help information'
complete -c fake-feature -n "__fish_use_subcommand" -s V -l version -d 'Print version information'
complete -c fake-feature -n "__fish_use_subcommand" -f -a "generate" -d 'Generate fake data'
complete -c fake-feature -n "__fish_use_subcommand" -f -a "completion" -d 'Generate shell completion file'
complete -c fake-feature -n "__fish_use_subcommand" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c fake-feature -n "__fish_seen_subcommand_from generate; and not __fish_seen_subcommand_from group; and not __fish_seen_subcommand_from label; and not __fish_seen_subcommand_from help" -l seed -r
complete -c fake-feature -n "__fish_seen_subcommand_from generate; and not __fish_seen_subcommand_from group; and not __fish_seen_subcommand_from label; and not __fish_seen_subcommand_from help" -s h -l help -d 'Print help information'
complete -c fake-feature -n "__fish_seen_subcommand_from generate; and not __fish_seen_subcommand_from group; and not __fish_seen_subcommand_from label; and not __fish_seen_subcommand_from help" -f -a "group" -d 'Feature group data'
complete -c fake-feature -n "__fish_seen_subcommand_from generate; and not __fish_seen_subcommand_from group; and not __fish_seen_subcommand_from label; and not __fish_seen_subcommand_from help" -f -a "label" -d 'Feature label data'
complete -c fake-feature -n "__fish_seen_subcommand_from generate; and not __fish_seen_subcommand_from group; and not __fish_seen_subcommand_from label; and not __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c fake-feature -n "__fish_seen_subcommand_from generate; and __fish_seen_subcommand_from group" -s I -l id-range -d 'ID range' -r
complete -c fake-feature -n "__fish_seen_subcommand_from generate; and __fish_seen_subcommand_from group" -l seed -r
complete -c fake-feature -n "__fish_seen_subcommand_from generate; and __fish_seen_subcommand_from group" -l version -d 'Print version information'
complete -c fake-feature -n "__fish_seen_subcommand_from generate; and __fish_seen_subcommand_from group" -s h -l help -d 'Print help information'
complete -c fake-feature -n "__fish_seen_subcommand_from generate; and __fish_seen_subcommand_from label" -s I -l id-range -r
complete -c fake-feature -n "__fish_seen_subcommand_from generate; and __fish_seen_subcommand_from label" -s T -l time-range -r
complete -c fake-feature -n "__fish_seen_subcommand_from generate; and __fish_seen_subcommand_from label" -s l -l limit -r
complete -c fake-feature -n "__fish_seen_subcommand_from generate; and __fish_seen_subcommand_from label" -l seed -r
complete -c fake-feature -n "__fish_seen_subcommand_from generate; and __fish_seen_subcommand_from label" -l version -d 'Print version information'
complete -c fake-feature -n "__fish_seen_subcommand_from generate; and __fish_seen_subcommand_from label" -s h -l help -d 'Print help information'
complete -c fake-feature -n "__fish_seen_subcommand_from generate; and __fish_seen_subcommand_from help" -l seed -r
complete -c fake-feature -n "__fish_seen_subcommand_from generate; and __fish_seen_subcommand_from help" -l version -d 'Print version information'
complete -c fake-feature -n "__fish_seen_subcommand_from generate; and __fish_seen_subcommand_from help" -s h -l help -d 'Print help information'
complete -c fake-feature -n "__fish_seen_subcommand_from completion" -s h -l help -d 'Print help information'
complete -c fake-feature -n "__fish_seen_subcommand_from help" -s h -l help -d 'Print help information'

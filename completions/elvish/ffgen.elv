
use builtin;
use str;

set edit:completion:arg-completer[ffgen] = [@words]{
    fn spaces [n]{
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand [text desc]{
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'ffgen'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'ffgen'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -V 'Print version information'
            cand --version 'Print version information'
            cand group 'Generate feature group data'
            cand label 'Generate feature label data'
            cand schema 'Generate feature store schema'
            cand list 'List available resources'
            cand completion 'Output shell completion code'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'ffgen;group'= {
            cand -I 'ID range'
            cand --id-range 'ID range'
            cand --seed 'Seed for the random generator'
            cand -s 'Schema file for ffgen'
            cand --schema 'Schema file for ffgen'
            cand --format 'Data format'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'ffgen;label'= {
            cand -I 'ID range'
            cand --id-range 'ID range'
            cand -T 'Label time range'
            cand --time-range 'Label time range'
            cand --limit 'Max entries to generate'
            cand --seed 'Seed for the random generator'
            cand -s 'Schema file for ffgen'
            cand --schema 'Schema file for ffgen'
            cand --format 'Data format'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'ffgen;schema'= {
            cand -s 'Schema file for ffgen'
            cand --schema 'Schema file for ffgen'
            cand --format 'Schema format'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'ffgen;list'= {
            cand -s 'Schema file for ffgen'
            cand --schema 'Schema file for ffgen'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'ffgen;completion'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'ffgen;help'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
    ]
    $completions[$command]
}

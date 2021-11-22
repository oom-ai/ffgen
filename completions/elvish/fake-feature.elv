
use builtin;
use str;

set edit:completion:arg-completer[fake-feature] = [@words]{
    fn spaces [n]{
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand [text desc]{
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'fake-feature'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'fake-feature'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -V 'Print version information'
            cand --version 'Print version information'
            cand generate 'Generate fake data'
            cand completion 'Generate shell completion file'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'fake-feature;generate'= {
            cand --seed 'seed'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand group 'Feature group data'
            cand label 'Feature label data'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'fake-feature;generate;group'= {
            cand -I 'ID range'
            cand --id-range 'ID range'
            cand --seed 'seed'
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'fake-feature;generate;label'= {
            cand -I 'I'
            cand --id-range 'id-range'
            cand -T 'T'
            cand --time-range 'time-range'
            cand -l 'l'
            cand --limit 'limit'
            cand --seed 'seed'
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'fake-feature;generate;help'= {
            cand --seed 'seed'
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'fake-feature;completion'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'fake-feature;help'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
    ]
    $completions[$command]
}

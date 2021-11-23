
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
            cand --seed 'seed'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -V 'Print version information'
            cand --version 'Print version information'
            cand group 'Generate feature group data'
            cand label 'Generate feature label data'
            cand completion 'Generate shell completion file'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'fake-feature;group'= {
            cand -I 'ID range'
            cand --id-range 'ID range'
            cand --seed 'seed'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'fake-feature;label'= {
            cand -I 'Label id range'
            cand --id-range 'Label id range'
            cand -T 'Label time range'
            cand --time-range 'Label time range'
            cand -l 'Max entries to generate'
            cand --limit 'Max entries to generate'
            cand --seed 'seed'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'fake-feature;completion'= {
            cand --seed 'seed'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'fake-feature;help'= {
            cand --seed 'seed'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
    ]
    $completions[$command]
}

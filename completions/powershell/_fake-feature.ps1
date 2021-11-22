
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'fake-feature' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'fake-feature'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-')) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'fake-feature' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('generate', 'generate', [CompletionResultType]::ParameterValue, 'Generate fake data')
            [CompletionResult]::new('completion', 'completion', [CompletionResultType]::ParameterValue, 'Generate shell completion file')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'fake-feature;generate' {
            [CompletionResult]::new('--seed', 'seed', [CompletionResultType]::ParameterName, 'seed')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('group', 'group', [CompletionResultType]::ParameterValue, 'Feature group data')
            [CompletionResult]::new('label', 'label', [CompletionResultType]::ParameterValue, 'Feature label data')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'fake-feature;generate;group' {
            [CompletionResult]::new('-I', 'I', [CompletionResultType]::ParameterName, 'ID range')
            [CompletionResult]::new('--id-range', 'id-range', [CompletionResultType]::ParameterName, 'ID range')
            [CompletionResult]::new('--seed', 'seed', [CompletionResultType]::ParameterName, 'seed')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'fake-feature;generate;label' {
            [CompletionResult]::new('-I', 'I', [CompletionResultType]::ParameterName, 'I')
            [CompletionResult]::new('--id-range', 'id-range', [CompletionResultType]::ParameterName, 'id-range')
            [CompletionResult]::new('-T', 'T', [CompletionResultType]::ParameterName, 'T')
            [CompletionResult]::new('--time-range', 'time-range', [CompletionResultType]::ParameterName, 'time-range')
            [CompletionResult]::new('-l', 'l', [CompletionResultType]::ParameterName, 'l')
            [CompletionResult]::new('--limit', 'limit', [CompletionResultType]::ParameterName, 'limit')
            [CompletionResult]::new('--seed', 'seed', [CompletionResultType]::ParameterName, 'seed')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'fake-feature;generate;help' {
            [CompletionResult]::new('--seed', 'seed', [CompletionResultType]::ParameterName, 'seed')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'fake-feature;completion' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'fake-feature;help' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}

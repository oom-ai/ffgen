
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'ffgen' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'ffgen'
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
        'ffgen' {
            [CompletionResult]::new('--seed', 'seed', [CompletionResultType]::ParameterName, 'Seed for the random generator')
            [CompletionResult]::new('-f', 'f', [CompletionResultType]::ParameterName, 'Schema file')
            [CompletionResult]::new('--file', 'file', [CompletionResultType]::ParameterName, 'Schema file')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('group', 'group', [CompletionResultType]::ParameterValue, 'Generate feature group data')
            [CompletionResult]::new('label', 'label', [CompletionResultType]::ParameterValue, 'Generate feature label data')
            [CompletionResult]::new('schema', 'schema', [CompletionResultType]::ParameterValue, 'Generate oomstore schema')
            [CompletionResult]::new('completion', 'completion', [CompletionResultType]::ParameterValue, 'Generate shell completion file')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'ffgen;group' {
            [CompletionResult]::new('-I', 'I', [CompletionResultType]::ParameterName, 'ID range')
            [CompletionResult]::new('--id-range', 'id-range', [CompletionResultType]::ParameterName, 'ID range')
            [CompletionResult]::new('--seed', 'seed', [CompletionResultType]::ParameterName, 'Seed for the random generator')
            [CompletionResult]::new('-f', 'f', [CompletionResultType]::ParameterName, 'Schema file')
            [CompletionResult]::new('--file', 'file', [CompletionResultType]::ParameterName, 'Schema file')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'ffgen;label' {
            [CompletionResult]::new('-I', 'I', [CompletionResultType]::ParameterName, 'ID range')
            [CompletionResult]::new('--id-range', 'id-range', [CompletionResultType]::ParameterName, 'ID range')
            [CompletionResult]::new('-T', 'T', [CompletionResultType]::ParameterName, 'Label time range')
            [CompletionResult]::new('--time-range', 'time-range', [CompletionResultType]::ParameterName, 'Label time range')
            [CompletionResult]::new('--limit', 'limit', [CompletionResultType]::ParameterName, 'Max entries to generate')
            [CompletionResult]::new('--seed', 'seed', [CompletionResultType]::ParameterName, 'Seed for the random generator')
            [CompletionResult]::new('-f', 'f', [CompletionResultType]::ParameterName, 'Schema file')
            [CompletionResult]::new('--file', 'file', [CompletionResultType]::ParameterName, 'Schema file')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'ffgen;schema' {
            [CompletionResult]::new('--seed', 'seed', [CompletionResultType]::ParameterName, 'Seed for the random generator')
            [CompletionResult]::new('-f', 'f', [CompletionResultType]::ParameterName, 'Schema file')
            [CompletionResult]::new('--file', 'file', [CompletionResultType]::ParameterName, 'Schema file')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'ffgen;completion' {
            [CompletionResult]::new('--seed', 'seed', [CompletionResultType]::ParameterName, 'Seed for the random generator')
            [CompletionResult]::new('-f', 'f', [CompletionResultType]::ParameterName, 'Schema file')
            [CompletionResult]::new('--file', 'file', [CompletionResultType]::ParameterName, 'Schema file')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'ffgen;help' {
            [CompletionResult]::new('--seed', 'seed', [CompletionResultType]::ParameterName, 'Seed for the random generator')
            [CompletionResult]::new('-f', 'f', [CompletionResultType]::ParameterName, 'Schema file')
            [CompletionResult]::new('--file', 'file', [CompletionResultType]::ParameterName, 'Schema file')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}

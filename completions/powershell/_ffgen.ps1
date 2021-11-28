
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
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('group', 'group', [CompletionResultType]::ParameterValue, 'Generate feature group data')
            [CompletionResult]::new('label', 'label', [CompletionResultType]::ParameterValue, 'Generate feature label data')
            [CompletionResult]::new('schema', 'schema', [CompletionResultType]::ParameterValue, 'Generate feature store schema')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List available resources')
            [CompletionResult]::new('completion', 'completion', [CompletionResultType]::ParameterValue, 'Output shell completion code')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'ffgen;group' {
            [CompletionResult]::new('-I', 'I', [CompletionResultType]::ParameterName, 'ID range')
            [CompletionResult]::new('--id-range', 'id-range', [CompletionResultType]::ParameterName, 'ID range')
            [CompletionResult]::new('-s', 's', [CompletionResultType]::ParameterName, 'Seed for the random generator')
            [CompletionResult]::new('--seed', 'seed', [CompletionResultType]::ParameterName, 'Seed for the random generator')
            [CompletionResult]::new('-r', 'r', [CompletionResultType]::ParameterName, 'Recipe file path')
            [CompletionResult]::new('--recipe', 'recipe', [CompletionResultType]::ParameterName, 'Recipe file path')
            [CompletionResult]::new('-f', 'f', [CompletionResultType]::ParameterName, 'Data format')
            [CompletionResult]::new('--format', 'format', [CompletionResultType]::ParameterName, 'Data format')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'ffgen;label' {
            [CompletionResult]::new('-I', 'I', [CompletionResultType]::ParameterName, 'ID range')
            [CompletionResult]::new('--id-range', 'id-range', [CompletionResultType]::ParameterName, 'ID range')
            [CompletionResult]::new('-T', 'T', [CompletionResultType]::ParameterName, 'Label time range')
            [CompletionResult]::new('--time-range', 'time-range', [CompletionResultType]::ParameterName, 'Label time range')
            [CompletionResult]::new('-l', 'l', [CompletionResultType]::ParameterName, 'Max entries to generate')
            [CompletionResult]::new('--limit', 'limit', [CompletionResultType]::ParameterName, 'Max entries to generate')
            [CompletionResult]::new('-s', 's', [CompletionResultType]::ParameterName, 'Seed for the random generator')
            [CompletionResult]::new('--seed', 'seed', [CompletionResultType]::ParameterName, 'Seed for the random generator')
            [CompletionResult]::new('-r', 'r', [CompletionResultType]::ParameterName, 'Recipe file path')
            [CompletionResult]::new('--recipe', 'recipe', [CompletionResultType]::ParameterName, 'Recipe file path')
            [CompletionResult]::new('-f', 'f', [CompletionResultType]::ParameterName, 'Data format')
            [CompletionResult]::new('--format', 'format', [CompletionResultType]::ParameterName, 'Data format')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'ffgen;schema' {
            [CompletionResult]::new('-r', 'r', [CompletionResultType]::ParameterName, 'Recipe file path')
            [CompletionResult]::new('--recipe', 'recipe', [CompletionResultType]::ParameterName, 'Recipe file path')
            [CompletionResult]::new('--format', 'format', [CompletionResultType]::ParameterName, 'Schema format')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'ffgen;list' {
            [CompletionResult]::new('-r', 'r', [CompletionResultType]::ParameterName, 'Recipe file path')
            [CompletionResult]::new('--recipe', 'recipe', [CompletionResultType]::ParameterName, 'Recipe file path')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'ffgen;completion' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'ffgen;help' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}

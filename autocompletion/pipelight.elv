
use builtin;
use str;

set edit:completion:arg-completer[pipelight] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'pipelight'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'pipelight'= {
            cand --config 'The folowing args are global arguments available for every subcommands'
            cand --attach 'Attach command to standard I/O'
            cand -v 'More output per occurrence'
            cand --verbose 'More output per occurrence'
            cand -q 'Less output per occurrence'
            cand --quiet 'Less output per occurrence'
            cand -u 'More output per occurrence'
            cand --internal-verbose 'More output per occurrence'
            cand -h 'Print help'
            cand --help 'Print help'
            cand enable 'enable'
            cand disable 'disable'
            cand run 'Run a pipeline (interactive)'
            cand trigger 'Manualy trigger pipelines'
            cand watch 'Launch a watcher on the working directory (debugging)'
            cand stop 'Stop the pipeline execution and its every child processes'
            cand ls 'List available pipelines with a few more useful informations'
            cand inspect 'Displays pipelines with the maximum verbosity level (interactive)'
            cand logs 'Display pipelines logs'
            cand completion 'Generate autocompletion script for most used shells (bash/zsh/fish)'
            cand init 'Create a `pipelight` config template file'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'pipelight;enable'= {
            cand --config 'The folowing args are global arguments available for every subcommands'
            cand --attach 'Attach command to standard I/O'
            cand -v 'More output per occurrence'
            cand --verbose 'More output per occurrence'
            cand -q 'Less output per occurrence'
            cand --quiet 'Less output per occurrence'
            cand -u 'More output per occurrence'
            cand --internal-verbose 'More output per occurrence'
            cand -h 'Print help'
            cand --help 'Print help'
            cand git-hooks 'Git hooks toggle'
            cand watcher 'Watcher toggle'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'pipelight;enable;git-hooks'= {
            cand --config 'The folowing args are global arguments available for every subcommands'
            cand --attach 'Attach command to standard I/O'
            cand -v 'More output per occurrence'
            cand --verbose 'More output per occurrence'
            cand -q 'Less output per occurrence'
            cand --quiet 'Less output per occurrence'
            cand -u 'More output per occurrence'
            cand --internal-verbose 'More output per occurrence'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'pipelight;enable;watcher'= {
            cand --config 'The folowing args are global arguments available for every subcommands'
            cand --attach 'Attach command to standard I/O'
            cand -v 'More output per occurrence'
            cand --verbose 'More output per occurrence'
            cand -q 'Less output per occurrence'
            cand --quiet 'Less output per occurrence'
            cand -u 'More output per occurrence'
            cand --internal-verbose 'More output per occurrence'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'pipelight;enable;help'= {
            cand git-hooks 'Git hooks toggle'
            cand watcher 'Watcher toggle'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'pipelight;enable;help;git-hooks'= {
        }
        &'pipelight;enable;help;watcher'= {
        }
        &'pipelight;enable;help;help'= {
        }
        &'pipelight;disable'= {
            cand --config 'The folowing args are global arguments available for every subcommands'
            cand --attach 'Attach command to standard I/O'
            cand -v 'More output per occurrence'
            cand --verbose 'More output per occurrence'
            cand -q 'Less output per occurrence'
            cand --quiet 'Less output per occurrence'
            cand -u 'More output per occurrence'
            cand --internal-verbose 'More output per occurrence'
            cand -h 'Print help'
            cand --help 'Print help'
            cand git-hooks 'Git hooks toggle'
            cand watcher 'Watcher toggle'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'pipelight;disable;git-hooks'= {
            cand --config 'The folowing args are global arguments available for every subcommands'
            cand --attach 'Attach command to standard I/O'
            cand -v 'More output per occurrence'
            cand --verbose 'More output per occurrence'
            cand -q 'Less output per occurrence'
            cand --quiet 'Less output per occurrence'
            cand -u 'More output per occurrence'
            cand --internal-verbose 'More output per occurrence'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'pipelight;disable;watcher'= {
            cand --config 'The folowing args are global arguments available for every subcommands'
            cand --attach 'Attach command to standard I/O'
            cand -v 'More output per occurrence'
            cand --verbose 'More output per occurrence'
            cand -q 'Less output per occurrence'
            cand --quiet 'Less output per occurrence'
            cand -u 'More output per occurrence'
            cand --internal-verbose 'More output per occurrence'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'pipelight;disable;help'= {
            cand git-hooks 'Git hooks toggle'
            cand watcher 'Watcher toggle'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'pipelight;disable;help;git-hooks'= {
        }
        &'pipelight;disable;help;watcher'= {
        }
        &'pipelight;disable;help;help'= {
        }
        &'pipelight;run'= {
            cand --flag 'Manualy set a flag/action to bypass environment computation'
            cand --config 'The folowing args are global arguments available for every subcommands'
            cand --attach 'Attach command to standard I/O'
            cand -v 'More output per occurrence'
            cand --verbose 'More output per occurrence'
            cand -q 'Less output per occurrence'
            cand --quiet 'Less output per occurrence'
            cand -u 'More output per occurrence'
            cand --internal-verbose 'More output per occurrence'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'pipelight;trigger'= {
            cand --flag 'Manualy set a flag/action to bypass environment computation'
            cand --config 'The folowing args are global arguments available for every subcommands'
            cand --attach 'Attach command to standard I/O'
            cand -v 'More output per occurrence'
            cand --verbose 'More output per occurrence'
            cand -q 'Less output per occurrence'
            cand --quiet 'Less output per occurrence'
            cand -u 'More output per occurrence'
            cand --internal-verbose 'More output per occurrence'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'pipelight;watch'= {
            cand --config 'The folowing args are global arguments available for every subcommands'
            cand --attach 'Attach command to standard I/O'
            cand -v 'More output per occurrence'
            cand --verbose 'More output per occurrence'
            cand -q 'Less output per occurrence'
            cand --quiet 'Less output per occurrence'
            cand -u 'More output per occurrence'
            cand --internal-verbose 'More output per occurrence'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'pipelight;stop'= {
            cand --flag 'Manualy set a flag/action to bypass environment computation'
            cand --config 'The folowing args are global arguments available for every subcommands'
            cand --attach 'Attach command to standard I/O'
            cand -v 'More output per occurrence'
            cand --verbose 'More output per occurrence'
            cand -q 'Less output per occurrence'
            cand --quiet 'Less output per occurrence'
            cand -u 'More output per occurrence'
            cand --internal-verbose 'More output per occurrence'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'pipelight;ls'= {
            cand --color 'Ignore the environment and enforce/disable colored output'
            cand --config 'The folowing args are global arguments available for every subcommands'
            cand --attach 'Attach command to standard I/O'
            cand --json 'Display logs in json format'
            cand -v 'More output per occurrence'
            cand --verbose 'More output per occurrence'
            cand -q 'Less output per occurrence'
            cand --quiet 'Less output per occurrence'
            cand -u 'More output per occurrence'
            cand --internal-verbose 'More output per occurrence'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'pipelight;inspect'= {
            cand --color 'Ignore the environment and enforce/disable colored output'
            cand --config 'The folowing args are global arguments available for every subcommands'
            cand --attach 'Attach command to standard I/O'
            cand --json 'Display logs in json format'
            cand -v 'More output per occurrence'
            cand --verbose 'More output per occurrence'
            cand -q 'Less output per occurrence'
            cand --quiet 'Less output per occurrence'
            cand -u 'More output per occurrence'
            cand --internal-verbose 'More output per occurrence'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'pipelight;logs'= {
            cand --color 'Ignore the environment and enforce/disable colored output'
            cand --config 'The folowing args are global arguments available for every subcommands'
            cand --attach 'Attach command to standard I/O'
            cand --json 'Display logs in json format'
            cand -v 'More output per occurrence'
            cand --verbose 'More output per occurrence'
            cand -q 'Less output per occurrence'
            cand --quiet 'Less output per occurrence'
            cand -u 'More output per occurrence'
            cand --internal-verbose 'More output per occurrence'
            cand -h 'Print help'
            cand --help 'Print help'
            cand rm 'Clear logs'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'pipelight;logs;rm'= {
            cand --config 'The folowing args are global arguments available for every subcommands'
            cand --attach 'Attach command to standard I/O'
            cand -v 'More output per occurrence'
            cand --verbose 'More output per occurrence'
            cand -q 'Less output per occurrence'
            cand --quiet 'Less output per occurrence'
            cand -u 'More output per occurrence'
            cand --internal-verbose 'More output per occurrence'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'pipelight;logs;help'= {
            cand rm 'Clear logs'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'pipelight;logs;help;rm'= {
        }
        &'pipelight;logs;help;help'= {
        }
        &'pipelight;completion'= {
            cand --name 'The shell name'
            cand --config 'The folowing args are global arguments available for every subcommands'
            cand --attach 'Attach command to standard I/O'
            cand -v 'More output per occurrence'
            cand --verbose 'More output per occurrence'
            cand -q 'Less output per occurrence'
            cand --quiet 'Less output per occurrence'
            cand -u 'More output per occurrence'
            cand --internal-verbose 'More output per occurrence'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'pipelight;init'= {
            cand --file 'The output file path'
            cand --template 'The template style'
            cand --config 'The folowing args are global arguments available for every subcommands'
            cand --attach 'Attach command to standard I/O'
            cand -v 'More output per occurrence'
            cand --verbose 'More output per occurrence'
            cand -q 'Less output per occurrence'
            cand --quiet 'Less output per occurrence'
            cand -u 'More output per occurrence'
            cand --internal-verbose 'More output per occurrence'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'pipelight;help'= {
            cand enable 'enable'
            cand disable 'disable'
            cand run 'Run a pipeline (interactive)'
            cand trigger 'Manualy trigger pipelines'
            cand watch 'Launch a watcher on the working directory (debugging)'
            cand stop 'Stop the pipeline execution and its every child processes'
            cand ls 'List available pipelines with a few more useful informations'
            cand inspect 'Displays pipelines with the maximum verbosity level (interactive)'
            cand logs 'Display pipelines logs'
            cand completion 'Generate autocompletion script for most used shells (bash/zsh/fish)'
            cand init 'Create a `pipelight` config template file'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'pipelight;help;enable'= {
            cand git-hooks 'Git hooks toggle'
            cand watcher 'Watcher toggle'
        }
        &'pipelight;help;enable;git-hooks'= {
        }
        &'pipelight;help;enable;watcher'= {
        }
        &'pipelight;help;disable'= {
            cand git-hooks 'Git hooks toggle'
            cand watcher 'Watcher toggle'
        }
        &'pipelight;help;disable;git-hooks'= {
        }
        &'pipelight;help;disable;watcher'= {
        }
        &'pipelight;help;run'= {
        }
        &'pipelight;help;trigger'= {
        }
        &'pipelight;help;watch'= {
        }
        &'pipelight;help;stop'= {
        }
        &'pipelight;help;ls'= {
        }
        &'pipelight;help;inspect'= {
        }
        &'pipelight;help;logs'= {
            cand rm 'Clear logs'
        }
        &'pipelight;help;logs;rm'= {
        }
        &'pipelight;help;completion'= {
        }
        &'pipelight;help;init'= {
        }
        &'pipelight;help;help'= {
        }
    ]
    $completions[$command]
}

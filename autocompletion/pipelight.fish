complete -c pipelight -n "__fish_use_subcommand" -l config -d 'The folowing args are global arguments available for every subcommands' -r -F
complete -c pipelight -n "__fish_use_subcommand" -l attach -d 'Attach command to standard I/O' -r
complete -c pipelight -n "__fish_use_subcommand" -s v -l verbose -d 'More output per occurrence'
complete -c pipelight -n "__fish_use_subcommand" -s q -l quiet -d 'Less output per occurrence'
complete -c pipelight -n "__fish_use_subcommand" -s u -l internal-verbose -d 'More output per occurrence'
complete -c pipelight -n "__fish_use_subcommand" -s h -l help -d 'Print help'
complete -c pipelight -n "__fish_use_subcommand" -f -a "enable"
complete -c pipelight -n "__fish_use_subcommand" -f -a "disable"
complete -c pipelight -n "__fish_use_subcommand" -f -a "run" -d 'Run a pipeline (interactive)'
complete -c pipelight -n "__fish_use_subcommand" -f -a "trigger" -d 'Manualy trigger pipelines'
complete -c pipelight -n "__fish_use_subcommand" -f -a "watch" -d 'Launch a watcher on the working directory (debugging)'
complete -c pipelight -n "__fish_use_subcommand" -f -a "stop" -d 'Stop the pipeline execution and its every child processes'
complete -c pipelight -n "__fish_use_subcommand" -f -a "ls" -d 'List available pipelines with a few more useful informations'
complete -c pipelight -n "__fish_use_subcommand" -f -a "inspect" -d 'Displays pipelines with the maximum verbosity level (interactive)'
complete -c pipelight -n "__fish_use_subcommand" -f -a "logs" -d 'Display pipelines logs'
complete -c pipelight -n "__fish_use_subcommand" -f -a "completion" -d 'Generate autocompletion script for most used shells (bash/zsh/fish)'
complete -c pipelight -n "__fish_use_subcommand" -f -a "init" -d 'Create a `pipelight` config template file'
complete -c pipelight -n "__fish_use_subcommand" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c pipelight -n "__fish_seen_subcommand_from enable; and not __fish_seen_subcommand_from git-hooks; and not __fish_seen_subcommand_from watcher; and not __fish_seen_subcommand_from help" -l config -d 'The folowing args are global arguments available for every subcommands' -r -F
complete -c pipelight -n "__fish_seen_subcommand_from enable; and not __fish_seen_subcommand_from git-hooks; and not __fish_seen_subcommand_from watcher; and not __fish_seen_subcommand_from help" -l attach -d 'Attach command to standard I/O' -r
complete -c pipelight -n "__fish_seen_subcommand_from enable; and not __fish_seen_subcommand_from git-hooks; and not __fish_seen_subcommand_from watcher; and not __fish_seen_subcommand_from help" -s v -l verbose -d 'More output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from enable; and not __fish_seen_subcommand_from git-hooks; and not __fish_seen_subcommand_from watcher; and not __fish_seen_subcommand_from help" -s q -l quiet -d 'Less output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from enable; and not __fish_seen_subcommand_from git-hooks; and not __fish_seen_subcommand_from watcher; and not __fish_seen_subcommand_from help" -s u -l internal-verbose -d 'More output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from enable; and not __fish_seen_subcommand_from git-hooks; and not __fish_seen_subcommand_from watcher; and not __fish_seen_subcommand_from help" -s h -l help -d 'Print help'
complete -c pipelight -n "__fish_seen_subcommand_from enable; and not __fish_seen_subcommand_from git-hooks; and not __fish_seen_subcommand_from watcher; and not __fish_seen_subcommand_from help" -f -a "git-hooks" -d 'Git hooks toggle'
complete -c pipelight -n "__fish_seen_subcommand_from enable; and not __fish_seen_subcommand_from git-hooks; and not __fish_seen_subcommand_from watcher; and not __fish_seen_subcommand_from help" -f -a "watcher" -d 'Watcher toggle'
complete -c pipelight -n "__fish_seen_subcommand_from enable; and not __fish_seen_subcommand_from git-hooks; and not __fish_seen_subcommand_from watcher; and not __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c pipelight -n "__fish_seen_subcommand_from enable; and __fish_seen_subcommand_from git-hooks" -l config -d 'The folowing args are global arguments available for every subcommands' -r -F
complete -c pipelight -n "__fish_seen_subcommand_from enable; and __fish_seen_subcommand_from git-hooks" -l attach -d 'Attach command to standard I/O' -r
complete -c pipelight -n "__fish_seen_subcommand_from enable; and __fish_seen_subcommand_from git-hooks" -s v -l verbose -d 'More output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from enable; and __fish_seen_subcommand_from git-hooks" -s q -l quiet -d 'Less output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from enable; and __fish_seen_subcommand_from git-hooks" -s u -l internal-verbose -d 'More output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from enable; and __fish_seen_subcommand_from git-hooks" -s h -l help -d 'Print help'
complete -c pipelight -n "__fish_seen_subcommand_from enable; and __fish_seen_subcommand_from watcher" -l config -d 'The folowing args are global arguments available for every subcommands' -r -F
complete -c pipelight -n "__fish_seen_subcommand_from enable; and __fish_seen_subcommand_from watcher" -l attach -d 'Attach command to standard I/O' -r
complete -c pipelight -n "__fish_seen_subcommand_from enable; and __fish_seen_subcommand_from watcher" -s v -l verbose -d 'More output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from enable; and __fish_seen_subcommand_from watcher" -s q -l quiet -d 'Less output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from enable; and __fish_seen_subcommand_from watcher" -s u -l internal-verbose -d 'More output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from enable; and __fish_seen_subcommand_from watcher" -s h -l help -d 'Print help'
complete -c pipelight -n "__fish_seen_subcommand_from enable; and __fish_seen_subcommand_from help; and not __fish_seen_subcommand_from git-hooks; and not __fish_seen_subcommand_from watcher; and not __fish_seen_subcommand_from help" -f -a "git-hooks" -d 'Git hooks toggle'
complete -c pipelight -n "__fish_seen_subcommand_from enable; and __fish_seen_subcommand_from help; and not __fish_seen_subcommand_from git-hooks; and not __fish_seen_subcommand_from watcher; and not __fish_seen_subcommand_from help" -f -a "watcher" -d 'Watcher toggle'
complete -c pipelight -n "__fish_seen_subcommand_from enable; and __fish_seen_subcommand_from help; and not __fish_seen_subcommand_from git-hooks; and not __fish_seen_subcommand_from watcher; and not __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c pipelight -n "__fish_seen_subcommand_from disable; and not __fish_seen_subcommand_from git-hooks; and not __fish_seen_subcommand_from watcher; and not __fish_seen_subcommand_from help" -l config -d 'The folowing args are global arguments available for every subcommands' -r -F
complete -c pipelight -n "__fish_seen_subcommand_from disable; and not __fish_seen_subcommand_from git-hooks; and not __fish_seen_subcommand_from watcher; and not __fish_seen_subcommand_from help" -l attach -d 'Attach command to standard I/O' -r
complete -c pipelight -n "__fish_seen_subcommand_from disable; and not __fish_seen_subcommand_from git-hooks; and not __fish_seen_subcommand_from watcher; and not __fish_seen_subcommand_from help" -s v -l verbose -d 'More output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from disable; and not __fish_seen_subcommand_from git-hooks; and not __fish_seen_subcommand_from watcher; and not __fish_seen_subcommand_from help" -s q -l quiet -d 'Less output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from disable; and not __fish_seen_subcommand_from git-hooks; and not __fish_seen_subcommand_from watcher; and not __fish_seen_subcommand_from help" -s u -l internal-verbose -d 'More output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from disable; and not __fish_seen_subcommand_from git-hooks; and not __fish_seen_subcommand_from watcher; and not __fish_seen_subcommand_from help" -s h -l help -d 'Print help'
complete -c pipelight -n "__fish_seen_subcommand_from disable; and not __fish_seen_subcommand_from git-hooks; and not __fish_seen_subcommand_from watcher; and not __fish_seen_subcommand_from help" -f -a "git-hooks" -d 'Git hooks toggle'
complete -c pipelight -n "__fish_seen_subcommand_from disable; and not __fish_seen_subcommand_from git-hooks; and not __fish_seen_subcommand_from watcher; and not __fish_seen_subcommand_from help" -f -a "watcher" -d 'Watcher toggle'
complete -c pipelight -n "__fish_seen_subcommand_from disable; and not __fish_seen_subcommand_from git-hooks; and not __fish_seen_subcommand_from watcher; and not __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c pipelight -n "__fish_seen_subcommand_from disable; and __fish_seen_subcommand_from git-hooks" -l config -d 'The folowing args are global arguments available for every subcommands' -r -F
complete -c pipelight -n "__fish_seen_subcommand_from disable; and __fish_seen_subcommand_from git-hooks" -l attach -d 'Attach command to standard I/O' -r
complete -c pipelight -n "__fish_seen_subcommand_from disable; and __fish_seen_subcommand_from git-hooks" -s v -l verbose -d 'More output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from disable; and __fish_seen_subcommand_from git-hooks" -s q -l quiet -d 'Less output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from disable; and __fish_seen_subcommand_from git-hooks" -s u -l internal-verbose -d 'More output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from disable; and __fish_seen_subcommand_from git-hooks" -s h -l help -d 'Print help'
complete -c pipelight -n "__fish_seen_subcommand_from disable; and __fish_seen_subcommand_from watcher" -l config -d 'The folowing args are global arguments available for every subcommands' -r -F
complete -c pipelight -n "__fish_seen_subcommand_from disable; and __fish_seen_subcommand_from watcher" -l attach -d 'Attach command to standard I/O' -r
complete -c pipelight -n "__fish_seen_subcommand_from disable; and __fish_seen_subcommand_from watcher" -s v -l verbose -d 'More output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from disable; and __fish_seen_subcommand_from watcher" -s q -l quiet -d 'Less output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from disable; and __fish_seen_subcommand_from watcher" -s u -l internal-verbose -d 'More output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from disable; and __fish_seen_subcommand_from watcher" -s h -l help -d 'Print help'
complete -c pipelight -n "__fish_seen_subcommand_from disable; and __fish_seen_subcommand_from help; and not __fish_seen_subcommand_from git-hooks; and not __fish_seen_subcommand_from watcher; and not __fish_seen_subcommand_from help" -f -a "git-hooks" -d 'Git hooks toggle'
complete -c pipelight -n "__fish_seen_subcommand_from disable; and __fish_seen_subcommand_from help; and not __fish_seen_subcommand_from git-hooks; and not __fish_seen_subcommand_from watcher; and not __fish_seen_subcommand_from help" -f -a "watcher" -d 'Watcher toggle'
complete -c pipelight -n "__fish_seen_subcommand_from disable; and __fish_seen_subcommand_from help; and not __fish_seen_subcommand_from git-hooks; and not __fish_seen_subcommand_from watcher; and not __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c pipelight -n "__fish_seen_subcommand_from run" -l flag -d 'Manualy set a flag/action to bypass environment computation' -r
complete -c pipelight -n "__fish_seen_subcommand_from run" -l config -d 'The folowing args are global arguments available for every subcommands' -r -F
complete -c pipelight -n "__fish_seen_subcommand_from run" -l attach -d 'Attach command to standard I/O' -r
complete -c pipelight -n "__fish_seen_subcommand_from run" -s v -l verbose -d 'More output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from run" -s q -l quiet -d 'Less output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from run" -s u -l internal-verbose -d 'More output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from run" -s h -l help -d 'Print help'
complete -c pipelight -n "__fish_seen_subcommand_from trigger" -l flag -d 'Manualy set a flag/action to bypass environment computation' -r
complete -c pipelight -n "__fish_seen_subcommand_from trigger" -l config -d 'The folowing args are global arguments available for every subcommands' -r -F
complete -c pipelight -n "__fish_seen_subcommand_from trigger" -l attach -d 'Attach command to standard I/O' -r
complete -c pipelight -n "__fish_seen_subcommand_from trigger" -s v -l verbose -d 'More output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from trigger" -s q -l quiet -d 'Less output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from trigger" -s u -l internal-verbose -d 'More output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from trigger" -s h -l help -d 'Print help'
complete -c pipelight -n "__fish_seen_subcommand_from watch" -l config -d 'The folowing args are global arguments available for every subcommands' -r -F
complete -c pipelight -n "__fish_seen_subcommand_from watch" -l attach -d 'Attach command to standard I/O' -r
complete -c pipelight -n "__fish_seen_subcommand_from watch" -s v -l verbose -d 'More output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from watch" -s q -l quiet -d 'Less output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from watch" -s u -l internal-verbose -d 'More output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from watch" -s h -l help -d 'Print help'
complete -c pipelight -n "__fish_seen_subcommand_from stop" -l flag -d 'Manualy set a flag/action to bypass environment computation' -r
complete -c pipelight -n "__fish_seen_subcommand_from stop" -l config -d 'The folowing args are global arguments available for every subcommands' -r -F
complete -c pipelight -n "__fish_seen_subcommand_from stop" -l attach -d 'Attach command to standard I/O' -r
complete -c pipelight -n "__fish_seen_subcommand_from stop" -s v -l verbose -d 'More output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from stop" -s q -l quiet -d 'Less output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from stop" -s u -l internal-verbose -d 'More output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from stop" -s h -l help -d 'Print help'
complete -c pipelight -n "__fish_seen_subcommand_from ls" -l color -d 'Ignore the environment and enforce/disable colored output' -r
complete -c pipelight -n "__fish_seen_subcommand_from ls" -l config -d 'The folowing args are global arguments available for every subcommands' -r -F
complete -c pipelight -n "__fish_seen_subcommand_from ls" -l attach -d 'Attach command to standard I/O' -r
complete -c pipelight -n "__fish_seen_subcommand_from ls" -l json -d 'Display logs in json format'
complete -c pipelight -n "__fish_seen_subcommand_from ls" -s v -l verbose -d 'More output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from ls" -s q -l quiet -d 'Less output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from ls" -s u -l internal-verbose -d 'More output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from ls" -s h -l help -d 'Print help'
complete -c pipelight -n "__fish_seen_subcommand_from inspect" -l color -d 'Ignore the environment and enforce/disable colored output' -r
complete -c pipelight -n "__fish_seen_subcommand_from inspect" -l config -d 'The folowing args are global arguments available for every subcommands' -r -F
complete -c pipelight -n "__fish_seen_subcommand_from inspect" -l attach -d 'Attach command to standard I/O' -r
complete -c pipelight -n "__fish_seen_subcommand_from inspect" -l json -d 'Display logs in json format'
complete -c pipelight -n "__fish_seen_subcommand_from inspect" -s v -l verbose -d 'More output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from inspect" -s q -l quiet -d 'Less output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from inspect" -s u -l internal-verbose -d 'More output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from inspect" -s h -l help -d 'Print help'
complete -c pipelight -n "__fish_seen_subcommand_from logs; and not __fish_seen_subcommand_from rm; and not __fish_seen_subcommand_from help" -l color -d 'Ignore the environment and enforce/disable colored output' -r
complete -c pipelight -n "__fish_seen_subcommand_from logs; and not __fish_seen_subcommand_from rm; and not __fish_seen_subcommand_from help" -l config -d 'The folowing args are global arguments available for every subcommands' -r -F
complete -c pipelight -n "__fish_seen_subcommand_from logs; and not __fish_seen_subcommand_from rm; and not __fish_seen_subcommand_from help" -l attach -d 'Attach command to standard I/O' -r
complete -c pipelight -n "__fish_seen_subcommand_from logs; and not __fish_seen_subcommand_from rm; and not __fish_seen_subcommand_from help" -l json -d 'Display logs in json format'
complete -c pipelight -n "__fish_seen_subcommand_from logs; and not __fish_seen_subcommand_from rm; and not __fish_seen_subcommand_from help" -s v -l verbose -d 'More output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from logs; and not __fish_seen_subcommand_from rm; and not __fish_seen_subcommand_from help" -s q -l quiet -d 'Less output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from logs; and not __fish_seen_subcommand_from rm; and not __fish_seen_subcommand_from help" -s u -l internal-verbose -d 'More output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from logs; and not __fish_seen_subcommand_from rm; and not __fish_seen_subcommand_from help" -s h -l help -d 'Print help'
complete -c pipelight -n "__fish_seen_subcommand_from logs; and not __fish_seen_subcommand_from rm; and not __fish_seen_subcommand_from help" -f -a "rm" -d 'Clear logs'
complete -c pipelight -n "__fish_seen_subcommand_from logs; and not __fish_seen_subcommand_from rm; and not __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c pipelight -n "__fish_seen_subcommand_from logs; and __fish_seen_subcommand_from rm" -l config -d 'The folowing args are global arguments available for every subcommands' -r -F
complete -c pipelight -n "__fish_seen_subcommand_from logs; and __fish_seen_subcommand_from rm" -l attach -d 'Attach command to standard I/O' -r
complete -c pipelight -n "__fish_seen_subcommand_from logs; and __fish_seen_subcommand_from rm" -s v -l verbose -d 'More output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from logs; and __fish_seen_subcommand_from rm" -s q -l quiet -d 'Less output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from logs; and __fish_seen_subcommand_from rm" -s u -l internal-verbose -d 'More output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from logs; and __fish_seen_subcommand_from rm" -s h -l help -d 'Print help'
complete -c pipelight -n "__fish_seen_subcommand_from logs; and __fish_seen_subcommand_from help; and not __fish_seen_subcommand_from rm; and not __fish_seen_subcommand_from help" -f -a "rm" -d 'Clear logs'
complete -c pipelight -n "__fish_seen_subcommand_from logs; and __fish_seen_subcommand_from help; and not __fish_seen_subcommand_from rm; and not __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c pipelight -n "__fish_seen_subcommand_from completion" -l name -d 'The shell name' -r -f -a "{bash	'',zsh	'',fish	'',elvish	''}"
complete -c pipelight -n "__fish_seen_subcommand_from completion" -l config -d 'The folowing args are global arguments available for every subcommands' -r -F
complete -c pipelight -n "__fish_seen_subcommand_from completion" -l attach -d 'Attach command to standard I/O' -r
complete -c pipelight -n "__fish_seen_subcommand_from completion" -s v -l verbose -d 'More output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from completion" -s q -l quiet -d 'Less output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from completion" -s u -l internal-verbose -d 'More output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from completion" -s h -l help -d 'Print help'
complete -c pipelight -n "__fish_seen_subcommand_from init" -l file -d 'The output file path' -r
complete -c pipelight -n "__fish_seen_subcommand_from init" -l template -d 'The template style' -r -f -a "{objects	'',helpers	'',javascript	'',toml	'',yaml	''}"
complete -c pipelight -n "__fish_seen_subcommand_from init" -l config -d 'The folowing args are global arguments available for every subcommands' -r -F
complete -c pipelight -n "__fish_seen_subcommand_from init" -l attach -d 'Attach command to standard I/O' -r
complete -c pipelight -n "__fish_seen_subcommand_from init" -s v -l verbose -d 'More output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from init" -s q -l quiet -d 'Less output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from init" -s u -l internal-verbose -d 'More output per occurrence'
complete -c pipelight -n "__fish_seen_subcommand_from init" -s h -l help -d 'Print help'
complete -c pipelight -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from enable; and not __fish_seen_subcommand_from disable; and not __fish_seen_subcommand_from run; and not __fish_seen_subcommand_from trigger; and not __fish_seen_subcommand_from watch; and not __fish_seen_subcommand_from stop; and not __fish_seen_subcommand_from ls; and not __fish_seen_subcommand_from inspect; and not __fish_seen_subcommand_from logs; and not __fish_seen_subcommand_from completion; and not __fish_seen_subcommand_from init; and not __fish_seen_subcommand_from help" -f -a "enable"
complete -c pipelight -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from enable; and not __fish_seen_subcommand_from disable; and not __fish_seen_subcommand_from run; and not __fish_seen_subcommand_from trigger; and not __fish_seen_subcommand_from watch; and not __fish_seen_subcommand_from stop; and not __fish_seen_subcommand_from ls; and not __fish_seen_subcommand_from inspect; and not __fish_seen_subcommand_from logs; and not __fish_seen_subcommand_from completion; and not __fish_seen_subcommand_from init; and not __fish_seen_subcommand_from help" -f -a "disable"
complete -c pipelight -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from enable; and not __fish_seen_subcommand_from disable; and not __fish_seen_subcommand_from run; and not __fish_seen_subcommand_from trigger; and not __fish_seen_subcommand_from watch; and not __fish_seen_subcommand_from stop; and not __fish_seen_subcommand_from ls; and not __fish_seen_subcommand_from inspect; and not __fish_seen_subcommand_from logs; and not __fish_seen_subcommand_from completion; and not __fish_seen_subcommand_from init; and not __fish_seen_subcommand_from help" -f -a "run" -d 'Run a pipeline (interactive)'
complete -c pipelight -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from enable; and not __fish_seen_subcommand_from disable; and not __fish_seen_subcommand_from run; and not __fish_seen_subcommand_from trigger; and not __fish_seen_subcommand_from watch; and not __fish_seen_subcommand_from stop; and not __fish_seen_subcommand_from ls; and not __fish_seen_subcommand_from inspect; and not __fish_seen_subcommand_from logs; and not __fish_seen_subcommand_from completion; and not __fish_seen_subcommand_from init; and not __fish_seen_subcommand_from help" -f -a "trigger" -d 'Manualy trigger pipelines'
complete -c pipelight -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from enable; and not __fish_seen_subcommand_from disable; and not __fish_seen_subcommand_from run; and not __fish_seen_subcommand_from trigger; and not __fish_seen_subcommand_from watch; and not __fish_seen_subcommand_from stop; and not __fish_seen_subcommand_from ls; and not __fish_seen_subcommand_from inspect; and not __fish_seen_subcommand_from logs; and not __fish_seen_subcommand_from completion; and not __fish_seen_subcommand_from init; and not __fish_seen_subcommand_from help" -f -a "watch" -d 'Launch a watcher on the working directory (debugging)'
complete -c pipelight -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from enable; and not __fish_seen_subcommand_from disable; and not __fish_seen_subcommand_from run; and not __fish_seen_subcommand_from trigger; and not __fish_seen_subcommand_from watch; and not __fish_seen_subcommand_from stop; and not __fish_seen_subcommand_from ls; and not __fish_seen_subcommand_from inspect; and not __fish_seen_subcommand_from logs; and not __fish_seen_subcommand_from completion; and not __fish_seen_subcommand_from init; and not __fish_seen_subcommand_from help" -f -a "stop" -d 'Stop the pipeline execution and its every child processes'
complete -c pipelight -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from enable; and not __fish_seen_subcommand_from disable; and not __fish_seen_subcommand_from run; and not __fish_seen_subcommand_from trigger; and not __fish_seen_subcommand_from watch; and not __fish_seen_subcommand_from stop; and not __fish_seen_subcommand_from ls; and not __fish_seen_subcommand_from inspect; and not __fish_seen_subcommand_from logs; and not __fish_seen_subcommand_from completion; and not __fish_seen_subcommand_from init; and not __fish_seen_subcommand_from help" -f -a "ls" -d 'List available pipelines with a few more useful informations'
complete -c pipelight -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from enable; and not __fish_seen_subcommand_from disable; and not __fish_seen_subcommand_from run; and not __fish_seen_subcommand_from trigger; and not __fish_seen_subcommand_from watch; and not __fish_seen_subcommand_from stop; and not __fish_seen_subcommand_from ls; and not __fish_seen_subcommand_from inspect; and not __fish_seen_subcommand_from logs; and not __fish_seen_subcommand_from completion; and not __fish_seen_subcommand_from init; and not __fish_seen_subcommand_from help" -f -a "inspect" -d 'Displays pipelines with the maximum verbosity level (interactive)'
complete -c pipelight -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from enable; and not __fish_seen_subcommand_from disable; and not __fish_seen_subcommand_from run; and not __fish_seen_subcommand_from trigger; and not __fish_seen_subcommand_from watch; and not __fish_seen_subcommand_from stop; and not __fish_seen_subcommand_from ls; and not __fish_seen_subcommand_from inspect; and not __fish_seen_subcommand_from logs; and not __fish_seen_subcommand_from completion; and not __fish_seen_subcommand_from init; and not __fish_seen_subcommand_from help" -f -a "logs" -d 'Display pipelines logs'
complete -c pipelight -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from enable; and not __fish_seen_subcommand_from disable; and not __fish_seen_subcommand_from run; and not __fish_seen_subcommand_from trigger; and not __fish_seen_subcommand_from watch; and not __fish_seen_subcommand_from stop; and not __fish_seen_subcommand_from ls; and not __fish_seen_subcommand_from inspect; and not __fish_seen_subcommand_from logs; and not __fish_seen_subcommand_from completion; and not __fish_seen_subcommand_from init; and not __fish_seen_subcommand_from help" -f -a "completion" -d 'Generate autocompletion script for most used shells (bash/zsh/fish)'
complete -c pipelight -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from enable; and not __fish_seen_subcommand_from disable; and not __fish_seen_subcommand_from run; and not __fish_seen_subcommand_from trigger; and not __fish_seen_subcommand_from watch; and not __fish_seen_subcommand_from stop; and not __fish_seen_subcommand_from ls; and not __fish_seen_subcommand_from inspect; and not __fish_seen_subcommand_from logs; and not __fish_seen_subcommand_from completion; and not __fish_seen_subcommand_from init; and not __fish_seen_subcommand_from help" -f -a "init" -d 'Create a `pipelight` config template file'
complete -c pipelight -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from enable; and not __fish_seen_subcommand_from disable; and not __fish_seen_subcommand_from run; and not __fish_seen_subcommand_from trigger; and not __fish_seen_subcommand_from watch; and not __fish_seen_subcommand_from stop; and not __fish_seen_subcommand_from ls; and not __fish_seen_subcommand_from inspect; and not __fish_seen_subcommand_from logs; and not __fish_seen_subcommand_from completion; and not __fish_seen_subcommand_from init; and not __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c pipelight -n "__fish_seen_subcommand_from help; and __fish_seen_subcommand_from enable; and not __fish_seen_subcommand_from git-hooks; and not __fish_seen_subcommand_from watcher" -f -a "git-hooks" -d 'Git hooks toggle'
complete -c pipelight -n "__fish_seen_subcommand_from help; and __fish_seen_subcommand_from enable; and not __fish_seen_subcommand_from git-hooks; and not __fish_seen_subcommand_from watcher" -f -a "watcher" -d 'Watcher toggle'
complete -c pipelight -n "__fish_seen_subcommand_from help; and __fish_seen_subcommand_from disable; and not __fish_seen_subcommand_from git-hooks; and not __fish_seen_subcommand_from watcher" -f -a "git-hooks" -d 'Git hooks toggle'
complete -c pipelight -n "__fish_seen_subcommand_from help; and __fish_seen_subcommand_from disable; and not __fish_seen_subcommand_from git-hooks; and not __fish_seen_subcommand_from watcher" -f -a "watcher" -d 'Watcher toggle'
complete -c pipelight -n "__fish_seen_subcommand_from help; and __fish_seen_subcommand_from logs; and not __fish_seen_subcommand_from rm" -f -a "rm" -d 'Clear logs'

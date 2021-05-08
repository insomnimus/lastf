
edit:completion:arg-completer[lf] = [@words]{
    fn spaces [n]{
        repeat $n ' ' | joins ''
    }
    fn cand [text desc]{
        edit:complex-candidate $text &display-suffix=' '(spaces (- 14 (wcswidth $text)))$desc
    }
    command = 'lf'
    for word $words[1:-1] {
        if (has-prefix $word '-') {
            break
        }
        command = $command';'$word
    }
    completions = [
        &'lf'= {
            cand -p 'path of the directory to evaluate'
            cand --path 'path of the directory to evaluate'
            cand -m 'sort by date last modified'
            cand --modified 'sort by date last modified'
            cand -c 'sort by date created'
            cand --created 'sort by date created'
            cand -a 'sort by date last accessed'
            cand --accessed 'sort by date last accessed'
            cand -o 'show oldest first'
            cand --oldest 'show oldest first'
            cand -d 'do not ignore hidden top level files and directories'
            cand --hidden 'do not ignore hidden top level files and directories'
            cand -n 'do not recursively calculate'
            cand --not-recursive 'do not recursively calculate'
            cand -t 'show the related date along with file names'
            cand --time 'show the related date along with file names'
            cand -F 'only display directories'
            cand --folders 'only display directories'
            cand -f 'only display regular files'
            cand --files 'only display regular files'
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
    ]
    $completions[$command]
}

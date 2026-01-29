# Search for a pattern within a file (formatted as JSON) or recursively in the directory.
export def search [
    pattern: string       # The text or regex pattern to search for
    --file (-f): path     # The specific file to open, convert to JSON, and search
    --list-files (-l)     # Only list the filenames that contain matches
    -A: int = 0           # Print NUM lines of trailing context after matching lines
    -C: int = 0           # Print NUM lines of output context
] {
    let flags = [
        "--ignore-case"
        "--no-ignore"
        "-A" $A
        "-C" $C
    ]

    if $list_files {
        return (rg -il --no-ignore $pattern)
    }

    if ($file | is-not-empty) {
        open $file | to json --indent 4 | rg ...$flags $pattern
    } else {
        rg ...$flags $pattern
    }
}
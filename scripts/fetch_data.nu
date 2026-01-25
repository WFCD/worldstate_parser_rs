

def main [
    --owner (-o): string = "WFCD"
    --repo (-r): string = "warframe-worldstate-data"
    --branch (-b): string = "master"
    --target_folder (-t): string = "data"
] {
    let api_url = $"https://api.github.com/repos/($owner)/($repo)/git/trees/($branch)?recursive=1"

    print $"Fetching file list from ($repo)..."

    let tree_data = http get $api_url

    let files_to_download = $tree_data.tree 
        | where { |row|
            let parsed_path = $row.path | path parse

            $parsed_path.parent == $target_folder and $parsed_path.extension == "json" and $row.type == "blob"
        } 

    print $"(ansi wb)Found (ansi gb)($files_to_download | length)(ansi wb) JSON files. Starting download...(ansi rst)"

    $files_to_download | par-each { |file|
        let remote_path = $file.path

        let raw_url = $"https://raw.githubusercontent.com/($owner)/($repo)/($branch)/($remote_path)"


        let parent_dir = ($remote_path | path dirname)
        if not ($parent_dir | path exists) {
            mkdir $parent_dir
        }

        print $"Downloading ($remote_path)..."

        try {
            http get --raw $raw_url | save --force $remote_path
        } catch {
            print $"Failed to download ($remote_path)"
        }
    }

    print "Download complete."
}
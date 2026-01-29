def main [
    --data-dir (-d): path = "./data"
    --out-dir (-o): path = "./assets"
    --file-name (-n): path = "relays.json"
] {
    let source = $data_dir | path join "solNodes.json"
    let dest = $out_dir | path join $file_name
    
    mkdir $out_dir

    open $source
        | transpose key val
        | where key has "HUB"
        | update val {
            $in.value | str replace --regex ' \(.*\)$' ''
        }
        | transpose -r -d
        | save -f $dest
}
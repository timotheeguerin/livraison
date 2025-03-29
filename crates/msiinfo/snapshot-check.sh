set -e

MSI="../livraison/temp/test/msi/basic/basic.msi"

mkdir -p snapshots/classic
cargo run -- export $MSI Dialog > snapshots/classic/Dialog.txt        
cargo run -- export $MSI Control > snapshots/classic/Control.txt        
cargo run -- export $MSI ControlEvent > snapshots/classic/ControlEvent.txt        
cargo run -- export $MSI EventMapping > snapshots/classic/EventMapping.txt        

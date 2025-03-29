set -e

cargo run -- export ../livraison/temp/test/msi/basic/basic.msi Dialog > dialog_new.txt        
cargo run -- export ../livraison/temp/test/msi/basic/basic.msi Control > controls_new.txt        
cargo run -- export ../livraison/temp/test/msi/basic/basic.msi ControlEvent > control_event_new.txt        
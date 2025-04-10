mod db;
mod handlers;
mod menu;
fn main() {
    db::init().expect("No se pudo inicializar la base de datos");
    menu::show_menu();
}

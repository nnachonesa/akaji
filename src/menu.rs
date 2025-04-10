use crate::handlers;
use crate::utils;
use dialoguer::{Input, Select};
use std::process::Command;

pub fn show_menu() {
    let options = vec![
        "Ingresar Movimiento",
        "Remover Movimiento",
        "Eliminar Movimiento",
        "Mostrar",
        "Opciones",
        "Salir",
    ];
    loop {
        utils::cleanconsole();
        let selection = Select::new()
            .with_prompt("Selecciona una opcion")
            .items(&options)
            .interact()
            .unwrap();

        match selection {
            0 => {
                let monto: f64 = Input::new()
                    .with_prompt("Ingresa el monto")
                    .interact_text()
                    .unwrap();

                let motivo: String = Input::new()
                    .with_prompt("Ingresa el motivo")
                    .interact_text()
                    .unwrap();

                let tip: String = Input::new()
                    .with_prompt("Ingresa la categoria")
                    .interact_text()
                    .unwrap();

                handlers::add(tip.to_string(), monto, motivo.to_string());
            }
            1 => {
                let monto: f64 = Input::new()
                    .with_prompt("Ingresa el monto")
                    .interact_text()
                    .unwrap();

                let motivo: String = Input::new()
                    .with_prompt("Ingresa el motivo")
                    .interact_text()
                    .unwrap();

                let tip: String = Input::new()
                    .with_prompt("Ingresa la categoria")
                    .interact_text()
                    .unwrap();

                handlers::remove(tip.to_string(), monto, motivo.to_string());
            }
            2 => {
                let id: i32 = Input::new()
                    .with_prompt("Ingresa la id del movimiento a eliminar")
                    .interact_text()
                    .unwrap();

                handlers::delete(id);
            }
            3 => {
                handlers::movements();
                let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
            }
            4 => {
                let option: i8 = Input::new()
                    .with_prompt(
                        "1. Resetear base de datos.\nIngresa un identificador de opcion valido",
                    )
                    .interact_text()
                    .unwrap();

                handlers::options(option);
                // opciones
            }
            5 => break, // salir
            _ => continue,
        }
    }
}

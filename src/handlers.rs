use crate::db;
use rusqlite::params;

pub fn add(category: String, amount: f64, reason: String) {
    let conn = db::init().unwrap();
    conn.execute(
        "INSERT INTO movimientos (tipo, monto, motivo, fecha, exclusive) VALUES (?1, ?2, ?3, datetime('now'), ?4)",
        params![category, amount, reason, "ingreso"],
    )
    .unwrap();
}

pub fn remove(category: String, amount: f64, reason: String) {
    let conn = db::init().unwrap();
    conn.execute(
        "INSERT INTO movimientos (tipo, monto, motivo, fecha, exclusive) VALUES (?1, ?2, ?3, datetime('now'), ?4)",
        params![category, amount, reason, "retiro"],
    )
    .unwrap();
}
pub fn delete(id: i32) {
    let conn = db::init().unwrap();
    conn.execute("DELETE FROM movimientos WHERE id=?1", params![id])
        .unwrap();
}

pub fn movements() {
    let conn = db::init().unwrap();
    let mut stmt = conn
        .prepare("SELECT id,tipo,monto,motivo,fecha,exclusive FROM movimientos")
        .unwrap();
    let mut total: f64 = 0.0;

    let movimientos = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, i32>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, f64>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
            ))
        })
        .unwrap();

    for mov in movimientos {
        let (id, tipo, monto, motivo, fecha, excl) = mov.unwrap();
        if excl == "ingreso" {
            total += monto
        };
        if excl == "retiro" {
            total -= monto
        };
        println!(
            "[{}] {} -{}- - ${} ({}) [{}]",
            id, tipo, excl, monto, motivo, fecha
        );
    }
    println!("MONTO TOTAL: ${}", total);
}

pub fn options(option: i8) {
    match option {
        1 => {
            // resetear/fixear base de datos
            let conn = db::init().unwrap();
            conn.execute("DROP TABLE movimientos", ()).unwrap();
        }
        _ => {
            println!("Opcion no reconocida")
        }
    }
}


extern crate sqlite;

use std::fs;
use std::path::Path;
use std::result::Result;

/// Ошибка SQLiteError - позволяет получить код события повлекшего к ошибке
#[derive(Debug)]
pub struct SQLiteError(i32);

// Объявляем основной альяс для `Result` для типа ошибки `SQLiteResult`.
type SQLiteResult<T> = Result<T, SQLiteError>;

/// Создаёт базу данных sqlite по указанному пути.
///
/// # Ошибки
///
/// Эта функция должна вернуть ошибку которая расскажет Вам о ситуации
/// повлекшей к этой ошибке. Есть несколько основных причин:
///
/// * Путь задан к каталогу, а не к файлу
/// * Файл занят другой программой
/// * Нет соответствующих библиотек
///
/// # Пример использования
///
/// ```rust
/// # fn foo() -> std::io::Result<()> {
/// mod data_base;
///
/// let res = data_base::create("/some/file/path.sqlite").unwrap();
///
/// # }
/// ```
pub fn create(file_path: &str) -> SQLiteResult<()> {

    // Если БД есть, то удаляем её
    if Path::new(file_path).exists() {

        let result = fs::remove_file(file_path);
        match result {
            Ok(_) => { /* Успех */ }
            Err(_) => return Err((SQLiteError(1))),
        }
    }

    // Создаём соединение и бд автокрейтиться
    let connection;
    let result = sqlite::open(file_path);
    match result {
        Ok(expr) => connection = expr,
        Err(_) => return Err((SQLiteError(2))),
    }

    let sql_commands = "\
    CREATE TABLE worked_time (id INTEGER, enter_time DOUBLE, leave_time DOUBLE, table_name_inuse TEXT);\
    CREATE TABLE holiday_time (id INTEGER, free_time DOUBLE, table_name_inuse TEXT);\
    CREATE TABLE docs (id INTEGER, general_director_name TEXT, head_of_department_name TEXT, responsible_for_attendance TEXT, my_name TEXT, body_text TEXT);\
    ";

    connection.execute(sql_commands).unwrap();

    Ok(())
}

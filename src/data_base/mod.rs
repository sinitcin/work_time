
extern crate sqlite;

use std::fs;
use std::path::Path;
use std::result::Result;

use std::num::ParseIntError;

/// Ошибка SQLiteError - позволяет получить код события повлекшего к ошибке
pub struct SQLiteError {
    code: i32,
}

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
            Some(_) => {/* Успех */},
            None => return Err(1),
        }
    }

    // Создаём соединение и бд автокрейтиться
    let connection = sqlite::open(file_path).unwrap();

    let sql_commands = "\
    CREATE TABLE users (name TEXT, age INTEGER);\
    CREATE TABLE users2 (name TEXT, age INTEGER);\
    CREATE TABLE users3 (name TEXT, age INTEGER);\
    ";

    connection.execute(sql_commands).unwrap();

    Ok(())
}

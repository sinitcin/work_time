
extern crate sqlite;

use std::fs;
use std::path::Path;
use std::result::Result;
use std::io::Error;

/// Ошибка SQLiteError - позволяет получить код события повлекшего к ошибке
#[derive(Debug)]
pub enum SQLiteError {
    CantRemoveFile,
    CantOpenConnect,
}

// Объявляем основной альяс для `Result` для типа ошибки `SQLiteResult`.
pub type SQLiteResult<T> = Result<T, SQLiteError>;

impl From<Error> for SQLiteError {
    #[allow(dead_code)]
    #[allow(unused_variables)]
    fn from(other: Error) -> Self {
        SQLiteError::CantRemoveFile
    }
}

impl From<sqlite::Error> for SQLiteError {
    #[allow(dead_code)]
    #[allow(unused_variables)]
    fn from(other: sqlite::Error) -> Self {
        SQLiteError::CantOpenConnect
    }
}


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

        try!(fs::remove_file(file_path));
    }

    // Создаём соединение и бд автокрейтиться
    let connection = try!(sqlite::open(file_path));

    let sql_commands = "\
    CREATE TABLE users (name TEXT, age INTEGER);\
    CREATE TABLE users2 (name TEXT, age INTEGER);\
    CREATE TABLE users3 (name TEXT, age INTEGER);\
    ";

    connection.execute(sql_commands).unwrap();

    Ok(())
}
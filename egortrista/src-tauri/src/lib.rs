//ИМПОРТИРУЕМ ТИПЫ ДЛЯ МИГРАЦИИ
use tauri_plugin_sql::{Migration,MigrationKind};
//aннотация для мобильных платформ
#[cfg_attr(mobile,tauri::mobile_entry_point)]

// главная функиця запуска приложеня
pub fn run (){
    //создаем список миграций
    let migration = vec![
        Migration{
            version: 1,
            description: "create_message_table",
            sql:include_str!("../migrations/0001_initial.sql"),
            kind: MigrationKind::Up,
        },
    ];

    //создаем сборщик приложения
    tauri::Builder::default()
    .plugin(tauri_plugin_sql::Builder::new().build())
    //подключаем sql плагин
        .plugin(
            tauri_plugin_sql::Builder::default()
            // связываем миграции с sql базой
                .add_migrations("sqlite:messenger.db", migration)
                .build(),
        )
    //создаем плагин опенер из стандартного шаблока tauri
        .plugin(tauri_plugin_opener::init())
    //запуск приложения
        .run(tauri::generate_context!())
    //если запуск с ошибкой сука немедленно вывести уведомление об етом
        .expect("ало кодинг дада ошибки");
    //




}
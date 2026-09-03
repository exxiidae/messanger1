--инициализация созднаия таблицы сообщений мессенджера
CREATE TABLE IF NOT EXISTS messages(
    --zoздаем id сообщения
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    -- ИМЯ АВТОРА СООБЩЕНЯ
    author TEXT NOT NULL,
    --САМ ТЕКСТ СООБЩЕНЯ
    body TEXT NOT NULL,
    --ВРЕМЯ ОТПРАВКИ СООБЩЕНЯ^^
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Включаем расширение для работы с UUID, если оно ещё не включено
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE recipes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,

    -- Docker Compose
    image VARCHAR(255) NOT NULL,
    ports TEXT[],
    environment TEXT[],
    volumes TEXT[],
    networks TEXT[],
    depends_on TEXT[],
    restart VARCHAR(50),
    command TEXT[],

    -- Extra files (массив объектов File в формате JSONB)
    files JSONB,

    -- .env
    env TEXT[],

    -- Notes
    notes TEXT[]
);

-- Индекс для быстрого поиска по имени сервиса
CREATE INDEX idx_recipes_name ON recipes(name);
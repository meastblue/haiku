-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS user_haikus (
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    haiku_id UUID REFERENCES haikus(id) ON DELETE CASCADE,
    is_read BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id, haiku_id)
);

CREATE INDEX idx_user_haikus_user_id ON user_haikus(user_id);
CREATE INDEX idx_user_haikus_haiku_id ON user_haikus(haiku_id);
CREATE INDEX idx_user_haikus_is_read ON user_haikus(is_read);

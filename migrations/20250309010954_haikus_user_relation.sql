-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE user_haikus (
     user_id UUID REFERENCES users(id),
     haiku_id UUID REFERENCES haikus(id),
     is_read BOOLEAN NOT NULL DEFAULT FALSE,
     created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
     updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
     PRIMARY KEY (user_id, haiku_id)
);

CREATE INDEX idx_user_haikus_user_id ON users(user_id);
CREATE INDEX idx_user_haikus_haiku_id ON users(haiku_id);
CREATE INDEX idx_user_haikus_is_read ON users(is_read);
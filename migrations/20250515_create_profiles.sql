-- Create profiles table
CREATE TABLE IF NOT EXISTS profiles (
    id TEXT PRIMARY KEY,
    bio TEXT NOT NULL, -- JSON array as text
    social_links TEXT NOT NULL, -- JSON array as text
    education TEXT NOT NULL, -- JSON array as text
    languages TEXT NOT NULL, -- JSON array as text
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

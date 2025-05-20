-- Add GitHub profiles table
CREATE TABLE IF NOT EXISTS github_profiles (
    id TEXT PRIMARY KEY,
    data TEXT NOT NULL,
    last_updated TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

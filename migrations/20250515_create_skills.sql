-- Create skills table
CREATE TABLE IF NOT EXISTS skills (
    id TEXT PRIMARY KEY,
    category TEXT NOT NULL,
    items TEXT NOT NULL, -- JSON array as text
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create experiences table
CREATE TABLE IF NOT EXISTS experiences (
    id TEXT PRIMARY KEY,
    company TEXT NOT NULL,
    position TEXT NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT,  -- Can be NULL for current positions
    description TEXT NOT NULL,
    technologies TEXT NOT NULL, -- JSON array as text
    highlights TEXT NOT NULL, -- JSON array as text
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

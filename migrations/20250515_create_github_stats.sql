-- Create github_stats table
CREATE TABLE IF NOT EXISTS github_stats (
    id TEXT PRIMARY KEY,
    username TEXT NOT NULL,
    repo_count INTEGER NOT NULL,
    followers INTEGER NOT NULL,
    contributions INTEGER NOT NULL,
    top_languages TEXT NOT NULL, -- JSON array as text
    recent_activity TEXT NOT NULL, -- JSON array as text
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

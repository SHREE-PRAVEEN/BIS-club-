CREATE TABLE IF NOT EXISTS team_members (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    position VARCHAR(255) NOT NULL,
    bio TEXT,
    email VARCHAR(255),
    phone VARCHAR(20),
    image_id INTEGER REFERENCES images(id) ON DELETE SET NULL,
    display_order INTEGER,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes
CREATE INDEX idx_team_members_order ON team_members(display_order);
CREATE INDEX idx_team_members_is_active ON team_members(is_active);
CREATE INDEX idx_team_members_image_id ON team_members(image_id);
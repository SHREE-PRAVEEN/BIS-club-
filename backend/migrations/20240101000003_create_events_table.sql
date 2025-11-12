-- Create events table
CREATE TABLE IF NOT EXISTS events (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    event_type VARCHAR(100),
    image_id INTEGER REFERENCES images(id) ON DELETE SET NULL,
    event_date DATE,
    start_time TIME,
    end_time TIME,
    location VARCHAR(255),
    is_published BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes
CREATE INDEX idx_events_type ON events(event_type);
CREATE INDEX idx_events_date ON events(event_date);
CREATE INDEX idx_events_image_id ON events(image_id);
CREATE INDEX idx_events_is_published ON events(is_published);
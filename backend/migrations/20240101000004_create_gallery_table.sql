-- Create gallery table for storing gallery items
CREATE TABLE IF NOT EXISTS gallery (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255),
    description TEXT,
    image_id INTEGER REFERENCES images(id) ON DELETE SET NULL,
    display_order INTEGER,
    is_featured BOOLEAN DEFAULT FALSE,
    gallery_category VARCHAR(100),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes
CREATE INDEX idx_gallery_featured ON gallery(is_featured);
CREATE INDEX idx_gallery_order ON gallery(display_order);
CREATE INDEX idx_gallery_category ON gallery(gallery_category);
CREATE INDEX idx_gallery_image_id ON gallery(image_id);
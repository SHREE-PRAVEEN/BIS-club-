-- Create images table to store all binary image data
CREATE TABLE IF NOT EXISTS images (
    id SERIAL PRIMARY KEY,
    image_name VARCHAR(255) NOT NULL UNIQUE,
    image_data BYTEA NOT NULL,
    content_type VARCHAR(50) NOT NULL,
    file_size INTEGER NOT NULL,
    category VARCHAR(100),
    description TEXT,
    uploaded_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for faster queries
CREATE INDEX idx_images_category ON images(category);
CREATE INDEX idx_images_name ON images(image_name);
CREATE INDEX idx_images_uploaded_at ON images(uploaded_at);
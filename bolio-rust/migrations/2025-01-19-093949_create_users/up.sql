-- CREATE TABLE users (
--     id SERIAL PRIMARY KEY,
--     username VARCHAR NOT NULL UNIQUE,
--     email VARCHAR NOT NULL UNIQUE,
--     password VARCHAR NOT NULL,
--     created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
--     updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
--     last_login TIMESTAMP WITH TIME ZONE, -- To store the time of the last login with timezone
--     session_id VARCHAR -- To store the current session ID
-- );

-- Users Table
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL UNIQUE,
    email VARCHAR NOT NULL UNIQUE,
    password VARCHAR NOT NULL, -- Store hashed password for security
    credit BIGINT NOT NULL DEFAULT 0 CHECK (credit >= 0), -- Remaining credits in bytes
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    last_login TIMESTAMP WITH TIME ZONE, -- Last login timestamp
    session_id VARCHAR -- Current session ID
);

-- Insert Default Admin User
INSERT INTO users (username, email, password, credit, created_at, updated_at)
VALUES (
    'admin', 
    'admin@bolio.cloud', 
    '$2b$12$JIM3X6u1csS8IC/z29UEsOQWDz2GJm9Edo9vaNg6a7QRc.sTe3ZQy', -- Replace with a hashed password in a real application
    1099511627776, 
    now(), 
    now()
);

-- Credit Transactions Table
CREATE TABLE credit_transactions (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE, -- Links to the users table
    transaction_type VARCHAR NOT NULL, -- Type of transaction (e.g., "top-up", "refund")
    amount BIGINT NOT NULL CHECK (amount > 0), -- Amount of credits added in bytes
    source VARCHAR NOT NULL, -- Payment source/type (e.g., "paypal", "stripe", "solana", "btc", "eth")
    transaction_id VARCHAR NOT NULL UNIQUE, -- Unique identifier for the transaction
    status VARCHAR NOT NULL, -- Transaction status (e.g., "pending", "completed", "failed")
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(), -- Transaction timestamp
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(), -- Last status update timestamp
    description TEXT -- Optional description of the transaction
);

-- File Conversion Transactions Table
CREATE TABLE conversion_transactions (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE, -- Links to the users table
    source_size BIGINT NOT NULL CHECK (source_size > 0), -- Size of the source file in bytes
    target_size BIGINT NOT NULL CHECK (target_size > 0), -- Size of the target file in bytes
    credit_used BIGINT NOT NULL GENERATED ALWAYS AS (source_size + target_size) STORED, -- Total credits used
    conversion_type VARCHAR NOT NULL, -- Type of transformation/conversion (e.g., "video-to-audio", "format-change")
    source_type VARCHAR NOT NULL, -- Source file type (e.g., "mp4", "avi", "wav")
    target_type VARCHAR NOT NULL, -- Target file type (e.g., "txt", "mp3", "mp4")
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now() -- Transaction timestamp
);
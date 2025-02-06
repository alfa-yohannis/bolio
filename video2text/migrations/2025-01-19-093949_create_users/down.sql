-- Drop File Conversion Transactions Table
DROP TABLE IF EXISTS conversion_transactions;

-- Drop Credit Transactions Table
DROP TABLE IF EXISTS credit_transactions;

-- Remove Default Admin User
DELETE FROM users WHERE username = 'admin' AND email = 'admin@bolio.cloud';

-- Drop Users Table
DROP TABLE IF EXISTS users;

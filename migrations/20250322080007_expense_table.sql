CREATE TYPE expense_category AS ENUM (
    'Groceries',
    'Leisure',
    'Electronics',
    'Utilities',
    'Clothing',
    'Health',
    'Others'
);

CREATE TABLE expenses (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    category expense_category NOT NULL,
    amount NUMERIC(10, 2) NOT NULL,
    description TEXT,
    expense_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- Optional indexes for performance
CREATE INDEX idx_expenses_user_id ON expenses(user_id);
CREATE INDEX idx_expenses_date ON expenses(expense_date);


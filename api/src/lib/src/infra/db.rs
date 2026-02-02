use rusqlite::Connection;
use tokio_rusqlite::Connection as AsyncConnection;

pub async fn init_db(path: &str) -> Result<AsyncConnection, tokio_rusqlite::Error> {
    let conn = AsyncConnection::open(path).await?;

    conn.call(|conn| {
        run_migrations(conn)?;
        Ok(())
    })
    .await?;

    Ok(conn)
}

fn run_migrations(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS cards (
            id TEXT PRIMARY KEY,
            user_email TEXT NOT NULL,
            name TEXT NOT NULL,
            card_type TEXT NOT NULL CHECK (card_type IN ('credit', 'debit')),
            credit_limit INTEGER,
            current_balance INTEGER NOT NULL DEFAULT 0
        );

        CREATE INDEX IF NOT EXISTS idx_cards_user_email ON cards(user_email);

        CREATE TABLE IF NOT EXISTS categories (
            id TEXT PRIMARY KEY,
            user_email TEXT,
            name TEXT NOT NULL,
            color TEXT
        );

        CREATE INDEX IF NOT EXISTS idx_categories_user_email ON categories(user_email);

        CREATE TABLE IF NOT EXISTS transactions (
            id TEXT PRIMARY KEY,
            user_email TEXT NOT NULL,
            card_id TEXT NOT NULL,
            category_id TEXT NOT NULL,
            amount INTEGER NOT NULL,
            description TEXT NOT NULL,
            transaction_type TEXT NOT NULL CHECK (transaction_type IN ('expense', 'income', 'payment')),
            date TEXT NOT NULL,
            FOREIGN KEY (card_id) REFERENCES cards(id),
            FOREIGN KEY (category_id) REFERENCES categories(id)
        );

        CREATE INDEX IF NOT EXISTS idx_transactions_user_email ON transactions(user_email);
        CREATE INDEX IF NOT EXISTS idx_transactions_card_id ON transactions(card_id);
        CREATE INDEX IF NOT EXISTS idx_transactions_date ON transactions(date);

        -- Insert default categories if they don't exist
        INSERT OR IGNORE INTO categories (id, user_email, name, color) VALUES
            ('1', NULL, 'Food & Dining', '#ef4444'),
            ('2', NULL, 'Transportation', '#f97316'),
            ('3', NULL, 'Shopping', '#eab308'),
            ('4', NULL, 'Entertainment', '#22c55e'),
            ('5', NULL, 'Bills & Utilities', '#3b82f6'),
            ('6', NULL, 'Health', '#8b5cf6'),
            ('7', NULL, 'Other', '#6b7280');
        "#,
    )?;

    Ok(())
}

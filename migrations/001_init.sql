-- PERSONNALISER : Adapter les colonnes selon vos besoins
CREATE TABLE IF NOT EXISTS contacts (
    id TEXT PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    phone TEXT,
    company TEXT,
    sector TEXT,
    status TEXT NOT NULL DEFAULT 'pending',
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);

-- PERSONNALISER : Ajouter des indexes selon votre usage
CREATE INDEX IF NOT EXISTS idx_contacts_status ON contacts(status);
CREATE INDEX IF NOT EXISTS idx_contacts_created_at ON contacts(created_at);
CREATE INDEX IF NOT EXISTS idx_contacts_email ON contacts(email);

-- PERSONNALISER : Table pour tracker les envois (optionnel)
CREATE TABLE IF NOT EXISTS email_exports (
    id TEXT PRIMARY KEY,
    recipient_email TEXT NOT NULL,
    contact_count INTEGER NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    created_at DATETIME NOT NULL,
    sent_at DATETIME
);

CREATE INDEX IF NOT EXISTS idx_exports_status ON email_exports(status);

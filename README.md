# SMP Backend

Backend API Rust pour l'application Android SMP Moules.

## Architecture

```
src/
├── main.rs              # Point d'entrée, assemblage
├── config/              # Configuration centralisée
│   └── mod.rs
├── domain/              # Types métier (indépendants HTTP)
│   └── mod.rs
├── email/               # Abstraction envoi email
│   ├── mod.rs
│   ├── provider.rs      # Trait EmailProvider
│   ├── resend.rs        # Implémentation Resend
│   └── templates.rs     # Templates HTML
├── handlers/            # Handlers HTTP (légers)
│   ├── mod.rs
│   ├── export_fiches.rs
│   ├── health.rs
│   └── history.rs
└── middleware/          # Auth, logging, etc.
    └── mod.rs
```

## Principes appliqués

1. **Séparation des responsabilités** - Chaque module a un rôle clair
2. **Injection de dépendances** - Le provider email est injecté via trait
3. **Configuration externalisée** - Tout vient des variables d'environnement
4. **Handlers légers** - Délèguent la logique aux services
5. **Testabilité** - Mock provider pour les tests unitaires

## Endpoints

| Méthode | Route | Description |
|---------|-------|-------------|
| GET | `/health` | Health check |
| POST | `/api/export-fiches` | Export fiches contacts par email |
| POST | `/api/send-history-email` | Envoi historique contacts |

## Configuration

Copiez `.env.example` vers `.env` et configurez :

```bash
cp .env.example .env
```

Variables requises :
- `API_KEY` - Clé d'authentification
- `RESEND_API_KEY` - Clé API Resend
- `DEFAULT_EXPORT_EMAIL` - Email destinataire par défaut

## Développement

```bash
# Lancer en développement
cargo run

# Tests
cargo test

# Build release
cargo build --release
```

## Déploiement Railway

1. Connecter le repo GitHub à Railway
2. Configurer les variables d'environnement
3. Railway détecte automatiquement Rust et build

Le `PORT` est automatiquement défini par Railway.

## Sécurité

- Toutes les routes `/api/*` requièrent le header `X-API-Key`
- Les emails sont envoyés via Resend (pas d'accès SMTP direct)
- Les photos sont transmises en base64 et attachées aux emails

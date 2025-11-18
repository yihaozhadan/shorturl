-- Namespace & database bootstrap
DEFINE NAMESPACE shorturl;
DEFINE DATABASE shorturl;

USE NAMESPACE shorturl;
USE DATABASE shorturl;

-- Url mapping table definition
DEFINE TABLE url_mappings SCHEMAFULL
    PERMISSIONS
        FOR select, create FULL
        FOR update, delete NONE;

DEFINE FIELD long_url ON TABLE url_mappings TYPE string
    ASSERT $value != NONE;

DEFINE FIELD short_code ON TABLE url_mappings TYPE string
    ASSERT $value != NONE;

DEFINE INDEX idx_url_mappings_short_code_unique
    ON TABLE url_mappings FIELDS short_code UNIQUE;

DEFINE INDEX idx_url_mappings_long_url_unique
    ON TABLE url_mappings FIELDS long_url UNIQUE;

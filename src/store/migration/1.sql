CREATE TABLE wonderful_info {
    id PRIMARY KEY,
    key text NOT NULL,
    value text NOT NULL
};

CREATE TABLE wonder_servers {
    id PRIMARY KEY,
    owner int NOT NULL,
    prefix text
};

CREATE TABLE roles {
    id BYTES PRIMARY KEY,
    name STRING NOT NULL UNIQUE (id, name),
    self_assignable BOOL DEFAULT true
};

CREATE INDEX wonderful_info_keys ON wonderful_info (key);
CREATE INDEX wonderful_server_self_assignable_roles ON wonderful_roles (id, name, self_assignable) WHERE self_assignable = true;

-- Your SQL goes here
CREATE TABLE internet_services (
    id SERIAL PRIMARY KEY,
    name VARCHAR(200),
    price DECIMAL(9,2),
    whitelist TEXT[],
    blacklist TEXT[],
    hourly int,
    created_at timestamp default now(),
    updated_at timestamp default now()
);
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name varchar(40) not null,
    token varchar(6) unique,
    role varchar,
    phone varchar(20) not null unique,
    ip varchar(64),
    mac_addr varchar(64),
    devices varchar(64),
    internet_service_id int,
    enabled boolean default false,
    password text not null,
    created_at timestamp default now(),
    updated_at timestamp default now(),
    constraint internet_service_fk
        foreign key(internet_service_id)
            references internet_services(id)

);

create table cache_info (
    id bigint primary key auto_increment,
    c_key varchar(200) not null,
    c_val varchar(200) not null,  
    create_time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

insert cache_info values(0, 'cache_helloworld.btc', 'c0ff5c133d424706ca76c4f39f98a0f876b8e04fdf0fde5b5a0934252342da68i0', current_date());
insert cache_info values(0, 'cache_ordinal.btc', '484720360a3e2c0e64e89e2e0ee7112e0860d5a73d95e6b96c1f6552317b87afi0', current_date());
insert cache_info values(0, 'cache_snakes.btc', '9e18f15c62eec4f881b35da4be77b4d6023c098ac2140ab43bdfc446a5139945i0', current_date());
insert cache_info values(0, 'cache_tcmart.btc', '247cee7ecadc47112df1cefb25df551d4197ae4a259ac69fa7d22b1100d3989ai0', current_date());
insert cache_info values(0, 'cache_bitcoinbrc20.btc', '983e1348a59092cae6bb42abebaa99cea114eec0531b224e8cfaaaf631ec63a2i0', current_date());


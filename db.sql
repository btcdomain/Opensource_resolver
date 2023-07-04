
create table cache_info (
    id bigint primary key auto_increment,
    c_key varchar(200) not null,
    c_val varchar(200) not null,  
    create_time DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

insert cache_info values(0, 'cache_helloworld.btc', 'c0ff5c133d424706ca76c4f39f98a0f876b8e04fdf0fde5b5a0934252342da68i0', current_date());
insert cache_info values(0, 'cache_ordinal.btc', '484720360a3e2c0e64e89e2e0ee7112e0860d5a73d95e6b96c1f6552317b87afi0', current_date());
insert cache_info values(0, 'cache_snakes.btc', '9e18f15c62eec4f881b35da4be77b4d6023c098ac2140ab43bdfc446a5139945i0', current_date());
insert cache_info values(0, 'cache_tcmart.btc', '5695fbe2a04366ec1b3a8dc7fb7fdef9c9b0b96d96b7cfc53adacd781f3cdf65i0', current_date());
insert cache_info values(0, 'cache_bitcoinbrc20.btc', '983e1348a59092cae6bb42abebaa99cea114eec0531b224e8cfaaaf631ec63a2i0', current_date());
insert cache_info values(0, 'cache_match3pizzas.btc', '3fe6a0fc6bacc07e79b774a4c91b1f698df0b63173ad26971dbd5c2e7b2a9c1ci0', current_date());



update domain_inscription_info set inscribe_num=12879080 where id = 4490;
update domain_inscription_info set inscribe_num=12879519 where id = 4491;
update domain_inscription_info set inscribe_num=12879739 where id = 4492;
update domain_inscription_info set inscribe_num=12879822 where id = 4493;
update domain_inscription_info set inscribe_num=12882000 where id = 4494;
update domain_inscription_info set inscribe_num=12883262 where id = 4495;
update domain_inscription_info set inscribe_num=12887229 where id = 4496;


update domain_inscription_info set inscribe_num=12887229 where id = 4495;

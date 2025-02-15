
-- 删除系统表
drop table if exists "tb01sys";
-- 创建系统表
create table "tb01sys" ("key" text, "value" text, constraint "tb01sys_pkey" primary key ("key"));
comment on table "tb01sys" is '系统';
comment on column "tb01sys"."key" is '键';
comment on column "tb01sys"."value" is '值';
-- 系统表基础数据
-- insert into "tb01sys" ("key", "value") values ('000', '001');

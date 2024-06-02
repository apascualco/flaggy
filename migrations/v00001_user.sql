create table user 
(
  id         varchar(36) not null primary key,
  email      varchar(156) not null,
  is_active  boolean not null,

  created_at datetime not null,
  updated_at datetime null,
  constraint user_email_unique unique (email)
) CHARACTER SET utf8mb4
  COLLATE utf8mb4_bin;

create table credential (
  user_id    varchar(36) not null primary key,
  password   varchar(256) not null,

  created_at datetime not null,
  updated_at datetime null,
  foreign key (user_id) references user (id)
) CHARACTER SET utf8mb4
  COLLATE utf8mb4_bin;

create table user_role
(
  role       varchar(256) not null,
  user_id    varchar(36) not null,
  is_active  boolean not null,

  created_at datetime not null,
  updated_at datetime null,
  primary key (role, user_id),
  foreign key (user_id) references user (id)
) CHARACTER SET utf8mb4
  COLLATE utf8mb4_bin;

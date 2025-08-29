-- 用户表
CREATE TABLE users (
                       id BIGINT PRIMARY KEY AUTO_INCREMENT,
                       user_id VARCHAR(50) NOT NULL,
                       username VARCHAR(50) NOT NULL,
                       password_hash VARCHAR(255) NOT NULL,
                       status TINYINT DEFAULT 1, /* 1:启用, 0:禁用 */
                       created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 会话表
CREATE TABLE user_sessions (
                               id BIGINT PRIMARY KEY AUTO_INCREMENT,
                               user_id VARCHAR(50) NOT NULL,
                               session_id CHAR(36) NOT NULL,   /* UUID */
                               browser_info VARCHAR(255),      /* 浏览器 / 设备信息 */
                               ip_address VARCHAR(50),
                               login_time DATETIME DEFAULT CURRENT_TIMESTAMP,
                               expire_time DATETIME,           /* 会话过期时间 */
                               is_active TINYINT DEFAULT 1     /* 是否有效（1 有效，0 无效） */
);

-- 长效登录token表
CREATE TABLE remember_tokens (
                                 id BIGINT PRIMARY KEY AUTO_INCREMENT,
                                 user_id VARCHAR(50) NOT NULL,
                                 token_hash VARCHAR(255) NOT NULL, /* 随机生成的安全 token */
                                 device_info VARCHAR(255),
                                 ip_address VARCHAR(50),
                                 expire_time DATETIME              /* 过期时间（例如30天） */
);

-- 角色表
CREATE TABLE roles (
                       role_id VARCHAR(50) PRIMARY KEY,          -- 角色 ID（UUID）
                       role_name VARCHAR(100) NOT NULL,          -- 角色名称，例如 "管理员"
                       description VARCHAR(255),                 -- 角色描述
                       status TINYINT DEFAULT 1,                 -- 1: 启用, 0: 禁用
                       created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 权限表
CREATE TABLE permissions (
                             permission_id VARCHAR(50) PRIMARY KEY,    -- 权限 ID
                             permission_name VARCHAR(100) NOT NULL,    -- 权限名称，例如 "用户管理"
                             permission_code VARCHAR(100) NOT NULL,    -- 唯一权限代码，例如 "user:read"
                             description VARCHAR(255),                 -- 说明
                             created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 角色与权限关联表（多对多）
CREATE TABLE role_permissions (
                                  id BIGINT PRIMARY KEY AUTO_INCREMENT,
                                  role_id VARCHAR(50) NOT NULL,             -- 对应 roles.role_id
                                  permission_id VARCHAR(50) NOT NULL,       -- 对应 permissions.permission_id
                                  created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 用户与角色关联表（多对多）
CREATE TABLE user_roles (
                            id BIGINT PRIMARY KEY AUTO_INCREMENT,
                            user_id VARCHAR(50) NOT NULL,              -- 对应 users.user_id
                            role_id VARCHAR(50) NOT NULL,              -- 对应 roles.role_id
                            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 插入初始化数据
INSERT INTO users (user_id, username, password_hash, status) VALUES
                                                                 ('01K3NC4V1PH3ZRTF2TWK6NG43P','admin', '\$2b\$12$TuLIBnkqAXpim4D5fAWiweyRNjHX7juch4mCPSEFr2JGY7mEpWvse', 1),
                                                                 ('01K3NC6HJZ8F24HD2R5E4XVKXQ','test', '\$2b\$12\$9TQlcpGBHdDhOgUEoJeP/en1oMHN4vlzDQ7kLMiknbybs9l7BL50K', 1);

-- 插入几个角色
INSERT INTO roles (role_id, role_name, description) VALUES
                                                        ('ROLE_ADMIN', '管理员', '系统最高权限'),
                                                        ('ROLE_USER', '普通用户', '基础访问权限');

-- 插入几个权限
INSERT INTO permissions (permission_id, permission_name, permission_code, description) VALUES
                                                                                           ('PERM_USER_MANAGE', '用户管理', 'user:manage', '管理用户的增删改查'),
                                                                                           ('PERM_VIEW_REPORT', '查看报表', 'report:view', '查看系统报表');

-- 给管理员分配所有权限
INSERT INTO role_permissions (role_id, permission_id) VALUES
                                                          ('ROLE_ADMIN', 'PERM_USER_MANAGE'),
                                                          ('ROLE_ADMIN', 'PERM_VIEW_REPORT');

-- 给普通用户分配查看报表权限
INSERT INTO role_permissions (role_id, permission_id) VALUES
    ('ROLE_USER', 'PERM_VIEW_REPORT');

-- 给 admin 用户分配管理员角色
INSERT INTO user_roles (user_id, role_id) VALUES
    ('01K3NC4V1PH3ZRTF2TWK6NG43P', 'ROLE_ADMIN');

-- 给 test 用户分配普通用户角色
INSERT INTO user_roles (user_id, role_id) VALUES
    ('01K3NC6HJZ8F24HD2R5E4XVKXQ', 'ROLE_USER');